use super::super::error::StructureError;
use crate::prop::{parse_as_prop, Prop};
use crate::type_expr::parse_to_type_expr;

use swc_common::{Span, Spanned};
use tser_ir::type_decl::{
    struct_::Field,
    struct_::Struct,
    union::{Union, UnionKind},
};

use swc_ecma_ast::{TsLit, TsLitType, TsType, TsTypeAliasDecl, TsUnionOrIntersectionType};
use tser_ir::type_decl::union::{ExternallyTaggedVariant, InternallyTaggedUnionBody};

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

fn try_get_externally_tagged_variant(
    members: &[Prop],
    span: Span,
) -> Result<ExternallyTaggedVariant, StructureError> {
    if members.len() != 1 {
        return Err(StructureError::new(
            span,
            "Externally tagged union variant must contain exactly one field",
        ));
    }
    let prop = &members[0];
    if prop.optional {
        return Err(StructureError::new(
            prop.ts_type_element.span(),
            "Field of externally tagged union variant must not be optional",
        ));
    }
    Ok(ExternallyTaggedVariant {
        name: prop.name.to_string(),
        ty: parse_to_type_expr(prop.ts_type)?,
    })
}

fn try_get_internally_tagged_variant(
    members: &mut Vec<Prop>,
) -> Result<Option<(String, Struct)>, StructureError> {
    struct DiscriminatorField {
        name: String,
        value: String,
    }
    fn find_and_remove_discriminator_field(
        props: &mut Vec<Prop>,
    ) -> Result<Option<DiscriminatorField>, StructureError> {
        for (idx, prop) in props.iter_mut().enumerate() {
            if let TsType::TsLitType(TsLitType {
                lit: TsLit::Str(lit_str),
                ..
            }) = prop.ts_type
            {
                if prop.optional {
                    return Err(StructureError::new(
                        prop.ts_type_element.span(),
                        "Union discriminator field must not be optional",
                    ));
                }
                let df = DiscriminatorField {
                    name: prop.name.to_string(),
                    value: lit_str.value.to_string(),
                };
                props.remove(idx);
                return Ok(Some(df));
            }
        }
        Ok(None)
    }
    match find_and_remove_discriminator_field(members)? {
        Some(discriminator_field) => {
            // internally tagged, like { type: "...", ... } | { type: "...", ... }
            let fields = members
                .iter()
                .map(Field::try_from)
                .collect::<Result<Vec<Field>, StructureError>>()?;
            Ok(Some((
                discriminator_field.name,
                Struct {
                    name: discriminator_field.value,
                    fields,
                },
            )))
        }
        None => Ok(None),
    }
}

fn detect_union_kind(mut members: Vec<Prop>, span: Span) -> Result<UnionKind, StructureError> {
    // Try finding discriminator field
    match try_get_internally_tagged_variant(&mut members)? {
        Some((tag_field, variant)) => {
            // internally tagged, like { type: "...", ... } | { type: "...", ... }
            Ok(UnionKind::InternallyTagged(InternallyTaggedUnionBody {
                tag_field,
                variants: vec![variant],
            }))
        }
        None => {
            // externally tagged, like { "foo": string } | { "bar": number }
            let variant = try_get_externally_tagged_variant(&members, span)?;
            Ok(UnionKind::ExternallyTagged(vec![variant]))
        }
    }
}

pub fn parse_union(type_alias_decl: &TsTypeAliasDecl) -> Result<Union, StructureError> {
    if let Some(type_params) = &type_alias_decl.type_params {
        return Err(type_params.span.into());
    }

    let name = type_alias_decl.id.sym.to_string();

    let mut kind: Option<UnionKind> = None;

    traverse_ts_union_variants(type_alias_decl.type_ann.as_ref(), &mut |ts_type| {
        let ts_type_lit = match ts_type {
            TsType::TsTypeLit(ts_type_lit) => ts_type_lit,
            other => return Err(StructureError::from(other.span())),
        };
        let mut members = ts_type_lit
            .members
            .iter()
            .map(parse_as_prop)
            .collect::<Result<Vec<Prop>, StructureError>>()?;
        match &mut kind {
            None => {
                // Using the first union member to detect union kind
                kind = Some(detect_union_kind(members, ts_type_lit.span)?)
            }
            Some(UnionKind::InternallyTagged(internally_tagged)) => {
                let (tag_field, variant) = match try_get_internally_tagged_variant(&mut members)? {
                    Some(some) => some,
                    None => {
                        return Err(StructureError::new(
                            ts_type_lit.span,
                            "The discriminator field is missing",
                        ))
                    }
                };
                if tag_field != internally_tagged.tag_field {
                    return Err(StructureError::new(
                        ts_type_lit.span,
                        "The discriminator field has a different name from the previous one",
                    ));
                }
                internally_tagged.variants.push(variant)
            }
            Some(UnionKind::ExternallyTagged(externally_tagged_variants)) => {
                let variant = try_get_externally_tagged_variant(&members, ts_type_lit.span)?;
                externally_tagged_variants.push(variant);
            }
        };
        Ok(())
    })?;
    match kind {
        None => Err(StructureError::new(
            type_alias_decl.type_ann.span(),
            "Union must have at least one variant",
        )),
        Some(kind) => Ok(Union { name, kind }),
    }
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
