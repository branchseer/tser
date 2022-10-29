#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Enum {
    pub name: String,
    pub kind: EnumKind,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum EnumKind {
    Strings(Vec<EnumValue<String>>),
    Integers(Vec<EnumValue<i64>>),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct EnumValue<T> {
    pub name: String,
    pub value: T,
}
