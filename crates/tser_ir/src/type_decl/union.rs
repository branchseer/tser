use crate::type_decl::struct_::{Field, Struct};
use crate::type_expr::TypeExpr;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct InternallyTaggedUnionBody {
    pub tag_field: String,
    pub variants: Vec<Struct>,
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
        let variants = self.variants.iter().map(|variant| {
            let only_field = get_only_field(variant)?;
            if only_field.name == data_field {
                Some(only_field)
            } else {
                None
            }
        }).collect::<Option<Vec<&Field>>>()?;
        Some(AdjacentlyTaggedUnionBody {
            tag_field: self.tag_field.clone(),
            data_field: data_field.to_string(), variants: variants.into_iter().cloned().collect(),
        })
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct AdjacentlyTaggedUnionBody {
    tag_field: String,
    data_field: String,
    variants: Vec<Field>, // AdjacentlyTagged variant can be optional
}

/// https://serde.rs/enum-representations.html
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum UnionKind {
    // ExternallyTagged variant can't be optional, thus (String, TypeExpr) instead of struct_::Field
    ExternallyTagged(Vec<(String, TypeExpr)>),
    InternallyTagged(InternallyTaggedUnionBody),
}


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Union {
    pub name: String,
    pub kind: UnionKind,
}
