use crate::error::StructureError;
use swc_common::Spanned;
use swc_ecma_ast::{
    TsEntityName, TsKeywordType, TsKeywordTypeKind, TsType, TsUnionOrIntersectionType,
};
use tser_ir::type_expr::primitive::Primitive;
use tser_ir::type_expr::{TypeExpr, TypeExprKind};

fn parse_ts_type_to_type_expr_kind(ts_type: &TsType) -> Result<TypeExprKind, StructureError> {
    Ok(match ts_type {
        TsType::TsKeywordType(keyword_type) => TypeExprKind::Primitive(match keyword_type.kind {
            TsKeywordTypeKind::TsBooleanKeyword => Primitive::Bool,
            TsKeywordTypeKind::TsNumberKeyword => Primitive::Number,
            TsKeywordTypeKind::TsStringKeyword => Primitive::String,
            _ => return Err(keyword_type.span().into()),
        }),
        TsType::TsArrayType(ts_array_type) => {
            let elem_type_expr = parse_to_type_expr(&ts_array_type.elem_type)?;
            TypeExprKind::ArrayOf(Box::new(elem_type_expr))
        }
        TsType::TsTypeRef(type_ref) => {
            if let Some(type_params) = &type_ref.type_params {
                return Err(type_params.span.into());
            }
            TypeExprKind::Identifier(match &type_ref.type_name {
                TsEntityName::Ident(ident) => ident.sym.to_string(),
                other => return Err(other.span().into()),
            })
        }
        other => {
            dbg!(other);
            return Err(other.span().into());
        }
    })
}

pub fn parse_to_type_expr(ts_type: &TsType) -> Result<TypeExpr, StructureError> {
    fn is_null(ts_type: &TsType) -> bool {
        matches!(
            ts_type,
            TsType::TsKeywordType(TsKeywordType {
                kind: TsKeywordTypeKind::TsNullKeyword,
                ..
            })
        )
    }
    match ts_type {
        TsType::TsUnionOrIntersectionType(TsUnionOrIntersectionType::TsUnionType(union_type)) => {
            // Parse "... | null"
            return match union_type.types.as_slice() {
                [type1, type2] => {
                    let type1 = type1.as_ref();
                    let type2 = type2.as_ref();
                    let ty = if is_null(type1) {
                        type2
                    } else if is_null(type2) {
                        type1
                    } else {
                        return Err(union_type.span.into());
                    };
                    let kind = parse_ts_type_to_type_expr_kind(ty)?;
                    Ok(TypeExpr {
                        nullable: true,
                        kind,
                    })
                }
                _ => Err(union_type.span.into()),
            };
        }
        TsType::TsParenthesizedType(parenthesized_type) => {
            parse_to_type_expr(parenthesized_type.type_ann.as_ref())
        }
        other => {
            let kind = parse_ts_type_to_type_expr_kind(other)?;
            Ok(TypeExpr {
                nullable: false,
                kind,
            })
        }
    }
}
#[cfg(test)]
mod tests {
    use super::super::ts_parser_from_source;
    use super::*;

    fn parse_type_expr(src: &str) -> TypeExpr {
        let mut parser = ts_parser_from_source(src);
        let ts_type = parser.parse_type().unwrap();
        parse_to_type_expr(ts_type.as_ref()).unwrap()
    }
    fn parse_type_expr_kind(src: &str) -> TypeExprKind {
        let expr = parse_type_expr(src);
        assert_eq!(expr.nullable, false);
        expr.kind
    }

    #[test]
    fn test_primitive() {
        assert_eq!(
            parse_type_expr_kind("string"),
            TypeExprKind::Primitive(Primitive::String)
        );
        assert_eq!(
            parse_type_expr_kind("boolean"),
            TypeExprKind::Primitive(Primitive::Bool)
        );
        assert_eq!(
            parse_type_expr_kind("number"),
            TypeExprKind::Primitive(Primitive::Number)
        );
    }
    #[test]
    fn test_identifier() {
        assert_eq!(
            parse_type_expr_kind("foo"),
            TypeExprKind::Identifier("foo".to_string())
        );
    }

    #[test]
    fn test_array() {
        assert_eq!(
            parse_type_expr_kind("number[]"),
            TypeExprKind::ArrayOf(Box::new(TypeExpr {
                nullable: false,
                kind: TypeExprKind::Primitive(Primitive::Number)
            }))
        );
    }

    #[test]
    fn test_parenthesized() {
        assert_eq!(
            parse_type_expr("(string | null)[]"),
            TypeExpr {
                nullable: false,
                kind: TypeExprKind::ArrayOf(Box::new(TypeExpr {
                    nullable: true,
                    kind: TypeExprKind::Primitive(Primitive::String)
                })),
            }
        );
    }

    #[test]
    fn test_nullable() {
        assert_eq!(
            parse_type_expr("string | null"),
            TypeExpr {
                nullable: true,
                kind: TypeExprKind::Primitive(Primitive::String),
            }
        );
        assert_eq!(
            parse_type_expr("null | string"),
            TypeExpr {
                nullable: true,
                kind: TypeExprKind::Primitive(Primitive::String),
            }
        );
    }
}
