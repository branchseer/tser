pub mod enum_;
pub mod struct_;
pub mod union;

use enum_::Enum;
use struct_::Struct;
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
