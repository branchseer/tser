mod lit;
mod interface;

use indexmap::{IndexMap, IndexSet};
use super::{BasicType, Lit, ParsingContext, Type, TypeDecl, TypeRef};
use crate::Result;
use swc_ecma_ast::{
    TsKeywordType, TsKeywordTypeKind, TsType,
    TsTypeAliasDecl, TsTypeParamDecl, TsTypeRef, TsUnionOrIntersectionType,
    TsTypeElement, Expr
};

impl<'a> ParsingContext<'a> {
    pub fn get_type_params(&self, type_param_decl: &TsTypeParamDecl) -> Result<Vec<String>> {
        type_param_decl
            .params
            .iter()
            .map(|param| {
                if param.default.is_some() {
                    Err(self.default_type_param_error(&param))
                } else {
                    Ok(param.name.sym.to_string())
                }
            })
            .collect()
    }
    fn parse_keyword_type(&self, keyword_type: &TsKeywordType) -> Result<Type> {
        Ok(match &keyword_type.kind {
            TsKeywordTypeKind::TsBooleanKeyword => Type::Basic(BasicType::Boolean),
            TsKeywordTypeKind::TsNumberKeyword => Type::Basic(BasicType::Number),
            TsKeywordTypeKind::TsStringKeyword => Type::Basic(BasicType::String),
            TsKeywordTypeKind::TsNullKeyword | TsKeywordTypeKind::TsUndefinedKeyword => {
                Type::Lit(Lit::Null)
            }
            TsKeywordTypeKind::TsAnyKeyword | TsKeywordTypeKind::TsUnknownKeyword => {
                Type::Unspecified
            }
            _ => return Err(self.unexpected_ast_node(keyword_type)),
        })
    }

    fn parse_type_ref(&self, type_ref: &TsTypeRef) -> Result<TypeRef> {
        let entity_path = self.parse_entity_path(&type_ref.type_name)?;
        let generic_args = match type_ref.type_params.as_ref() {
            Some(type_params) => type_params
                .params
                .iter()
                .map(|param_type| self.parse_type(param_type.as_ref()))
                .collect::<Result<Vec<Type>>>()?,
            None => vec![],
        };
        Ok(TypeRef {
            entity_path,
            generic_args,
        })
    }
    pub fn parse_type(&self, ty: &TsType) -> Result<Type> {
        Ok(match ty {
            TsType::TsKeywordType(keyword_type) => self.parse_keyword_type(keyword_type)?,
            TsType::TsTypeRef(ts_type_ref) => Type::Ref(self.parse_type_ref(ts_type_ref)?),
            TsType::TsLitType(ts_lit_type) => Type::Lit(self.parse_ts_lit(&ts_lit_type.lit)?),
            TsType::TsArrayType(ts_array_type) => {
                Type::Array(Box::new(self.parse_type(ts_array_type.elem_type.as_ref())?))
            }
            TsType::TsTupleType(ts_tuple_type) => Type::Tuple(
                ts_tuple_type
                    .elem_types
                    .iter()
                    .map(|ts_tuple_elem| self.parse_type(&ts_tuple_elem.ty))
                    .collect::<Result<Vec<Type>>>()?,
            ),
            // TsType::TsOptionalType(ts_optional_type) => {
            //     let mut ty = self.parse_type(ts_optional_type.type_ann.as_ref())?;
            //     ty.make_optional();
            //     ty
            // }
            TsType::TsUnionOrIntersectionType(TsUnionOrIntersectionType::TsUnionType(ts_union_type)) => {
                let union_elems = ts_union_type.types
                    .iter()
                    .map(|elem| self.parse_type(elem.as_ref()))
                    .collect::<Result<IndexSet<Type>>>()?;
                Type::Union(union_elems.into())
            },
            TsType::TsTypeLit(ts_type_lit) => {
                let props = self.parse_type_elements(ts_type_lit.members.as_slice())?;
                Type::Interface(props)
            }
            _ => return Err(self.unexpected_spanned(ty)),
        })
    }
    pub fn parse_type_alias(&self, type_alias_decl: TsTypeAliasDecl) -> Result<TypeDecl> {
        let name = type_alias_decl.id.sym.to_string();
        let type_params = match type_alias_decl.type_params.as_ref() {
            Some(some) => self.get_type_params(some)?,
            None => vec![],
        };
        let ty = self.parse_type(type_alias_decl.type_ann.as_ref())?;

        Ok(TypeDecl {
            name,
            type_params,
            ty,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::super::{
        parse, BasicType, EntityPath, Lit, Module, ModuleItem, Type, TypeDecl, TypeRef,
    };
    use indexmap::{indexset, indexmap};
    #[test]
    fn test_basic_alias() {
        let module = parse(Default::default(), "export type Hello = string").unwrap();
        assert_eq!(
            module,
            Module {
                items: vec![ModuleItem::Type(TypeDecl {
                    name: "Hello".to_string(),
                    type_params: vec![],
                    ty: Type::Basic(BasicType::String)
                })]
            }
        );
    }

    #[test]
    fn test_generic_alias() {
        let module = parse(Default::default(), "export type X<T> = Y").unwrap();
        assert_eq!(
            module,
            Module {
                items: vec![ModuleItem::Type(TypeDecl {
                    name: "X".to_string(),
                    type_params: vec!["T".to_string()],
                    ty: Type::Ref(TypeRef {
                        entity_path: EntityPath {
                            ancestors: vec![],
                            name: "Y".to_string()
                        },
                        generic_args: vec![]
                    })
                })]
            }
        );
    }

    #[test]
    fn test_type_args() {
        let module = parse(Default::default(), "export type X = Y<string>").unwrap();
        assert_eq!(
            module,
            Module {
                items: vec![ModuleItem::Type(TypeDecl {
                    name: "X".to_string(),
                    type_params: vec![],
                    ty: Type::Ref(TypeRef {
                        entity_path: EntityPath {
                            ancestors: vec![],
                            name: "Y".to_string()
                        },
                        generic_args: vec![Type::Basic(BasicType::String)]
                    })
                })]
            }
        );
    }

    #[test]
    fn test_interface_lit() {
        let module = parse(Default::default(), "export type X = { a: boolean, opt?: string }").unwrap();
        assert_eq!(
            module,
            Module {
                items: vec![ModuleItem::Type(TypeDecl {
                    name: "X".to_string(),
                    type_params: vec![],
                    ty: Type::Interface(indexmap! {
                        "a".to_string() => Type::Basic(BasicType::Boolean),
                        "opt".to_string() => Type::Union(indexset![
                            Type::Basic(BasicType::String),
                            Type::Lit(Lit::Null)
                        ].into()),
                    }.into())
                })]
            }
        );
    }

    #[test]
    fn test_array_type() {
        let module = parse(Default::default(), "export type X = string[]").unwrap();
        assert_eq!(
            module,
            Module {
                items: vec![ModuleItem::Type(TypeDecl {
                    name: "X".to_string(),
                    type_params: vec![],
                    ty: Type::Array(Box::new(Type::Basic(BasicType::String)))
                })]
            }
        );
    }

    #[test]
    fn test_tuple_type() {
        let module = parse(Default::default(), "export type X = [string, null]").unwrap();
        assert_eq!(
            module,
            Module {
                items: vec![ModuleItem::Type(TypeDecl {
                    name: "X".to_string(),
                    type_params: vec![],
                    ty: Type::Tuple(vec![
                        Type::Basic(BasicType::String),
                        Type::Lit(Lit::Null)
                    ])
                })]
            }
        );
    }

    #[test]
    fn test_union() {
        let module = parse(Default::default(), "export type X = boolean | 'hello' | null").unwrap();
        assert_eq!(
            module,
            Module {
                items: vec![ModuleItem::Type(TypeDecl {
                    name: "X".to_string(),
                    type_params: vec![],
                    ty: Type::Union(indexset![
                        Type::Basic(BasicType::Boolean),
                        Type::Lit(Lit::String("hello".to_string())),
                        Type::Lit(Lit::Null)
                    ].into())
                })]
            }
        );
    }
}
