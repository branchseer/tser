use crate::type_expr::TypeExpr;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Field {
    pub name: String,
    pub ty: TypeExpr,
    pub optional: bool,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Struct {
    pub name: String,
    pub fields: Vec<Field>,
}
