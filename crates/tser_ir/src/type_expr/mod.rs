pub mod primitive;

use primitive::Primitive;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum TypeExprKind {
    Identifier(String),
    ArrayOf(Box<TypeExpr>),
    Primitive(Primitive),
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct TypeExpr {
    pub nullable: bool,
    pub kind: TypeExprKind,
}
