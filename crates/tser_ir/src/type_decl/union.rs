use crate::type_decl::struct_::{Field, Struct};
use crate::type_expr::TypeExpr;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct InternallyTaggedUnionBody {
    pub tag_field: String,
    pub variants: Vec<Struct>, // Struct::name is the value of tag_field
}

fn get_only_field(s: &Struct) -> Option<&Field> {
    if s.fields.len() == 1 {
        Some(&s.fields[0])
    } else {
        None
    }
}
impl InternallyTaggedUnionBody {
    pub fn as_adjacently_tagged(&self) -> Option<AdjacentlyTaggedUnionBody> {
        let data_field = get_only_field(self.variants.first()?)?.name.as_str();
        let variants = self
            .variants
            .iter()
            .map(|variant| {
                let only_field = get_only_field(variant)?;
                if only_field.name == data_field {
                    Some(AdjacentlyTaggedUnionVariant {
                        optional: only_field.optional,
                        ty: only_field.ty.clone(),
                    })
                } else {
                    None
                }
            })
            .collect::<Option<Vec<AdjacentlyTaggedUnionVariant>>>()?;
        Some(AdjacentlyTaggedUnionBody {
            tag_field: self.tag_field.clone(),
            data_field: data_field.to_string(),
            variants,
        })
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct AdjacentlyTaggedUnionVariant {
    optional: bool,
    ty: TypeExpr,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct AdjacentlyTaggedUnionBody {
    tag_field: String,
    data_field: String,
    variants: Vec<AdjacentlyTaggedUnionVariant>, // AdjacentlyTagged variant can be optional
}

// Unlike struct_::Field, ExternallyTaggedVariant can't be optional
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ExternallyTaggedVariant {
    pub name: String,
    pub ty: TypeExpr,
}

/// https://serde.rs/enum-representations.html
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum UnionKind {
    ExternallyTagged(Vec<ExternallyTaggedVariant>),
    InternallyTagged(InternallyTaggedUnionBody),
    // TODO: untagged
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Union {
    pub name: String,
    pub kind: UnionKind,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::type_expr::primitive::Primitive;
    use crate::type_expr::TypeExprKind;

    fn generate_struct_variants(names: &[(&str, &[&str])]) -> Vec<Struct> {
        names
            .iter()
            .map(|(struct_name, field_names)| Struct {
                name: struct_name.to_string(),
                fields: field_names
                    .iter()
                    .map(|field_name| Field {
                        name: field_name.to_string(),
                        ty: TypeExpr {
                            nullable: false,
                            kind: TypeExprKind::Primitive(Primitive::String),
                        },
                        optional: false,
                    })
                    .collect(),
            })
            .collect()
    }

    #[test]
    fn adjacently_tagged_simple() {
        let internally_tagged = InternallyTaggedUnionBody {
            tag_field: "t".to_string(),
            variants: generate_struct_variants(&[("a", &["c"]), ("b", &["c"])]),
        };
        let adjacently_tagged = internally_tagged.as_adjacently_tagged().unwrap();
        assert_eq!(adjacently_tagged.tag_field, "t");
        assert_eq!(adjacently_tagged.data_field, "c");
    }
    #[test]
    fn adjacently_tagged_multiple_fields() {
        let internally_tagged = InternallyTaggedUnionBody {
            tag_field: "t".to_string(),
            variants: generate_struct_variants(&[("a", &["c"]), ("b", &["c", "b"])]),
        };
        assert_eq!(internally_tagged.as_adjacently_tagged(), None);
    }
    #[test]
    fn adjacently_tagged_different_field_names() {
        let internally_tagged = InternallyTaggedUnionBody {
            tag_field: "t".to_string(),
            variants: generate_struct_variants(&[("a", &["c"]), ("b", &["d"])]),
        };
        assert_eq!(internally_tagged.as_adjacently_tagged(), None);
    }
}
