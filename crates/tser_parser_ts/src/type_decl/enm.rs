use crate::error::StructureError;
use swc_common::Spanned;
use swc_ecma_ast::{Expr, Lit, Number, Str, TsEnumDecl, TsEnumMember, TsEnumMemberId};
use tser_ir::type_decl::enm::{Enum, EnumKind, EnumValue};

enum AnyEnumValue {
    String(String),
    Int(i64),
}

fn parse_enum_member(
    ts_enum_member: &TsEnumMember,
) -> Result<EnumValue<Option<AnyEnumValue>>, StructureError> {
    let name = match &ts_enum_member.id {
        TsEnumMemberId::Str(Str { value, .. }) => value.to_string(),
        TsEnumMemberId::Ident(ident) => ident.sym.to_string(),
    };
    let value = match ts_enum_member.init.as_ref() {
        Some(expr) => match expr.as_ref() {
            Expr::Lit(Lit::Str(Str { value, .. })) => Some(AnyEnumValue::String(value.to_string())),
            Expr::Lit(Lit::Num(Number { value, .. })) => {
                let value = *value;
                let int_value = value as i64;
                if int_value as f64 != value {
                    return Err(expr.span().into());
                }
                Some(AnyEnumValue::Int(int_value))
            }
            // TODO: support automatic int values
            other => return Err(other.span().into()),
        },
        None => None,
    };
    Ok(EnumValue { name, value })
}

pub fn parse_enum(ts_enum: &TsEnumDecl) -> Result<Enum, StructureError> {
    let name = ts_enum.id.sym.to_string();
    let mut member_iter = ts_enum.members.as_slice().iter();
    let mut enum_kind = if let Some(first_member) = member_iter.next() {
        let EnumValue { name, value } = parse_enum_member(first_member)?;
        match value {
            Some(AnyEnumValue::String(string_value)) => {
                EnumKind::Strings(vec![EnumValue::<String> {
                    name,
                    value: string_value,
                }])
            }
            Some(AnyEnumValue::Int(int_value)) => EnumKind::Integers(vec![EnumValue::<i64> {
                name,
                value: int_value,
            }]),
            None => EnumKind::Integers(vec![EnumValue::<i64> { name, value: 0 }]),
        }
    } else {
        return Err(ts_enum.span().into());
    };
    for member in member_iter {
        let EnumValue { name, value } = parse_enum_member(member)?;
        match (value, &mut enum_kind) {
            (Some(AnyEnumValue::Int(int_value)), EnumKind::Integers(cases)) => {
                cases.push(EnumValue {
                    name,
                    value: int_value,
                })
            }
            (None, EnumKind::Integers(cases)) => cases.push(EnumValue {
                name,
                value: cases.last().unwrap().value + 1,
            }),
            (Some(AnyEnumValue::String(string_value)), EnumKind::Strings(cases)) => {
                cases.push(EnumValue {
                    name,
                    value: string_value,
                })
            }
            _ => return Err(member.span.into()),
        }
    }
    Ok(Enum {
        name,
        kind: enum_kind,
    })
}

#[cfg(test)]
mod tests {
    use super::super::super::test_utils::parse_src_as_decl;
    use super::*;
    use assert_matches::assert_matches;
    use swc_ecma_ast::Decl;

    fn parse_src_as_enum(src: &str) -> Result<Enum, StructureError> {
        let decl = parse_src_as_decl(src);
        let ts_enum = assert_matches!(&decl, Decl::TsEnum(ts_enum) => ts_enum.as_ref());
        parse_enum(ts_enum)
    }

    #[test]
    fn test_string_enum() {
        assert_eq!(
            parse_src_as_enum(r"enum Foo { X = 'A', Y = 'B', }").unwrap(),
            Enum {
                name: "Foo".to_string(),
                kind: EnumKind::Strings(vec![
                    EnumValue {
                        name: 'X'.to_string(),
                        value: 'A'.to_string(),
                    },
                    EnumValue {
                        name: 'Y'.to_string(),
                        value: 'B'.to_string(),
                    },
                ])
            }
        );
    }

    #[test]
    fn test_enum_without_lit() {
        assert_eq!(
            parse_src_as_enum("enum Foo { X, Y }").unwrap(),
            Enum {
                name: "Foo".to_string(),
                kind: EnumKind::Integers(vec![
                    EnumValue {
                        name: 'X'.to_string(),
                        value: 0,
                    },
                    EnumValue {
                        name: 'Y'.to_string(),
                        value: 1,
                    },
                ])
            }
        );
    }

    #[test]
    fn test_int_enum() {
        assert_eq!(
            parse_src_as_enum(r"enum Foo { X = 4, Y = 2 }").unwrap(),
            Enum {
                name: "Foo".to_string(),
                kind: EnumKind::Integers(vec![
                    EnumValue {
                        name: 'X'.to_string(),
                        value: 4,
                    },
                    EnumValue {
                        name: 'Y'.to_string(),
                        value: 2,
                    },
                ])
            }
        );
    }
    #[test]
    fn test_lit_str_name() {
        assert_eq!(
            parse_src_as_enum(r"enum Foo { 'X' = 0 }").unwrap(),
            Enum {
                name: "Foo".to_string(),
                kind: EnumKind::Integers(vec![EnumValue {
                    name: 'X'.to_string(),
                    value: 0,
                }])
            }
        );
    }
    #[test]
    fn test_mixed_types() {
        assert_matches!(parse_src_as_enum(r"enum Foo { X = 0, Y = 'Y' }"), Err(_));
    }
    #[test]
    fn test_float_number() {
        assert_matches!(parse_src_as_enum(r"enum Foo { X = 1.1 }"), Err(_));
    }
}
