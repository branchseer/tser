use super::super::error::StructureError;
use crate::prop::{parse_as_prop, Prop};
use crate::type_expr::parse_to_type_expr;
use crate::UNION_DISCRIMINATION_FIELD;
use std::process::id;
use swc_common::Spanned;
use tser_ir::type_decl::{
    struct_::Field,
    struct_::Struct,
    union::{Union, UnionKind},
};

use swc_ecma_ast::{
    TsLit, TsLitType, TsType, TsTypeAliasDecl, TsTypeLit, TsUnionOrIntersectionType,
};
use tser_ir::type_decl::union::InternallyTaggedUnionBody;

fn traverse_ts_union_variants<E>(
    ts_type: &TsType,
    cb: &mut impl FnMut(&TsType) -> Result<(), E>,
) -> Result<(), E> {
    match ts_type {
        TsType::TsUnionOrIntersectionType(TsUnionOrIntersectionType::TsUnionType(
            ts_union_type,
        )) => {
            for child in ts_union_type.types.as_slice() {
                traverse_ts_union_variants(child.as_ref(), cb)?
            }
            Ok(())
        }
        other => cb(other),
    }
}

struct DiscriminatorField {
    name: String,
    value: String,
}
fn try_parse_discriminator_field(
    prop: &Prop,
) -> Option<Result<DiscriminatorField, StructureError>> {
    if let TsType::TsLitType(TsLitType {
        lit: TsLit::Str(lit_str),
        ..
    }) = prop.ts_type
    {
        if prop.optional {
            return Some(Err(StructureError::new(
                prop.ts_type_element.span(),
                "Union discriminator field must not be optional",
            )));
        }
        Some(Ok(DiscriminatorField {
            name: prop.name.to_string(),
            value: lit_str.value.to_string(),
        }))
    } else {
        None
    }
}

// fn find_map_and_remove<Item, Mapped, F: FnMut(&mut Item) -> Option<Mapped>>(vec: &mut Vec<Item>, f: F) {
//     let iter_result = for (idx, item) in vec.iter_mut().enumerate() {
//         match f(item) {
//             Some(mapped) => break (idx, mapped),
//             None => continue
//         }
//     };
// }

fn parse_union_kind(ts_type_lit: &TsTypeLit) -> Result<UnionKind, StructureError> {
    let mut members = ts_type_lit
        .members
        .iter()
        .map(parse_as_prop)
        .collect::<Result<Vec<Prop>, StructureError>>()?;

    // Try find discriminator_field
    let discriminator_field_with_idx = members
        .iter()
        .enumerate()
        .find_map(|(idx, member)| Some(try_parse_discriminator_field(member)?.map(|df| (idx, df))))
        .transpose()?;

    match discriminator_field_with_idx {
        Some((idx, discriminator_field)) => {
            // { type: "...", ... }
            members.remove(idx); // Exclude the discriminated field from members
            let tag_field = discriminator_field.name;
            let variant_fields = members
                .into_iter()
                .map(Field::try_from)
                .collect::<Result<Vec<Field>, StructureError>>()?;
            let variant_struct = Struct {
                name: discriminator_field.value,
                fields: variant_fields,
            };
            Ok(UnionKind::InternallyTagged(InternallyTaggedUnionBody {
                tag_field, variants: vec![variant_struct]
            }))
        }
        None => {
            todo!()
        }
    }
}

pub fn parse_union(type_alias_decl: &TsTypeAliasDecl) -> Result<Union, StructureError> {
    if let Some(type_params) = &type_alias_decl.type_params {
        return Err(type_params.span.into());
    }

    let name = type_alias_decl.id.sym.to_string();
    let mut variants: Vec<Struct> = vec![];

    traverse_ts_union_variants(
        type_alias_decl.type_ann.as_ref(),
        &mut |ts_type| match ts_type {
            TsType::TsTypeLit(ts_type_lit) => {
                let mut variant_name: Option<String> = None;
                let mut fields: Vec<Field> = vec![];
                for member in &ts_type_lit.members {
                    let prop = parse_as_prop(member)?;
                    if prop.name == UNION_DISCRIMINATION_FIELD {
                        if prop.optional {
                            return Err(member.span().into());
                        }
                        if variant_name.is_some() {
                            return Err(member.span().into());
                        }
                        match prop.ts_type {
                            TsType::TsLitType(TsLitType {
                                lit: TsLit::Str(lit_str),
                                ..
                            }) => variant_name = Some(lit_str.value.to_string()),
                            other => return Err(other.span().into()),
                        }
                    } else {
                        let field_type_expr = parse_to_type_expr(prop.ts_type)?;
                        fields.push(Field {
                            name: prop.name,
                            optional: prop.optional,
                            ty: field_type_expr,
                        })
                    }
                }
                let variant_name =
                    variant_name.ok_or_else(|| StructureError::from(ts_type_lit.span()))?;
                variants.push(Struct {
                    name: variant_name,
                    fields,
                });
                Ok(())
            }
            other => Err(StructureError::from(other.span())),
        },
    )?;
    todo!()
    // Ok(Union { name, variants })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::parse_src_as_decl;
    use assert_matches::assert_matches;
    use swc_ecma_ast::Decl;
    use tser_ir::type_expr::primitive::Primitive;
    use tser_ir::type_expr::{TypeExpr, TypeExprKind};

    fn parse_src_as_union(src: &str) -> Result<Union, StructureError> {
        let decl = parse_src_as_decl(src);
        let ts_typealias_decl =
            assert_matches!(&decl, Decl::TsTypeAlias(ts_typealias) => ts_typealias.as_ref());
        parse_union(ts_typealias_decl)
    }

    #[test]
    fn multiple_variants() {
        assert_eq!(
            parse_src_as_union(
                r"type Bar = { type: 'hello', val: number } |
                        { type: 'empty' } |
                        { type: 'maybe_a_string', val?: string }"
            )
            .unwrap(),
            Union {
                name: "Bar".to_string(),
                kind: UnionKind::InternallyTagged(InternallyTaggedUnionBody {
                    tag_field: "type".to_string(),
                    variants: vec![
                        Struct {
                            name: "hello".to_string(),
                            fields: vec![Field {
                                name: "val".to_string(),
                                ty: TypeExpr {
                                    nullable: false,
                                    kind: TypeExprKind::Primitive(Primitive::Number),
                                },
                                optional: false
                            }],
                        },
                        Struct {
                            name: "empty".to_string(),
                            fields: vec![],
                        },
                        Struct {
                            name: "maybe_a_string".to_string(),
                            fields: vec![Field {
                                name: "val".to_string(),
                                ty: TypeExpr {
                                    nullable: false,
                                    kind: TypeExprKind::Primitive(Primitive::String),
                                },
                                optional: true
                            }],
                        },
                    ],
                })
            },
        );
    }

    #[test]
    fn single_variant() {
        assert_eq!(
            parse_src_as_union("type Foo = { type: 'a' }").unwrap(),
            Union {
                name: "Foo".to_string(),
                kind: UnionKind::InternallyTagged(InternallyTaggedUnionBody {
                    tag_field: "type".to_string(),
                    variants: vec![Struct {
                        name: "a".to_string(),
                        fields: vec![],
                    }]
                }),
            },
        );
    }
}
