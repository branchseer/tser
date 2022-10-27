pub mod enm;
pub mod st;
pub mod union;

use enm::Enum;
use st::Struct;
use union::Union;

pub enum TypeDecl {
    Enum(Enum),
    Struct(Struct),
    Union(Union),
}

impl TypeDecl {
    pub fn name(&self) -> &str {
        match self {
            TypeDecl::Struct(st) => &st.name,
            TypeDecl::Enum(enm) => &enm.name,
            TypeDecl::Union(union) => &union.name,
        }
    }
}
