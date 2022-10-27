use crate::type_decl::TypeDecl;

pub struct Body {
    pub unary: Option<TypeDecl>,
    pub stream_item: Option<TypeDecl>,
}

pub struct Method {
    pub request: Body,
    pub response: Body,
}

pub struct Service {
    pub name: String,
    pub methods: Vec<Method>,
}
