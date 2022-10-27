use crate::type_decl::st::{Struct};

// { type: ..., content: ... }
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Union {
    pub name: String,
    pub variants: Vec<Struct>,
}
