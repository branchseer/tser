pub mod service;
pub mod type_decl;
pub mod type_expr;

use service::Service;
use type_decl::TypeDecl;

pub enum Item {
    TypeDecl(TypeDecl),
    Service(Service),
}

impl Item {
    pub fn name(&self) -> &str {
        match self {
            Self::Service(service) => &service.name,
            Self::TypeDecl(type_decl) => type_decl.name(),
        }
    }
}

pub struct File {
    pub items: Vec<Item>,
}
