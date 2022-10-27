use tser_block::{Block, block};
use tser_ir::type_expr::primitive::Primitive;
use crate::{CodeGen, Enum, EnumValueType, Struct, Union};

pub struct SwiftCodeGen;

fn ident(id: &str) -> String {
    if KEYWORDS.contains(&id) {
        format!("`${id}`")
    } else {
        id.to_string()
    }
}
fn quote(string: &str) -> String {
    format!("\"{}\"", string.escape_default())
}

const PROTOCOLS: &str = "Codable, Equatable, Hashable";

impl CodeGen for SwiftCodeGen {
    fn head(&self) -> Block {
        block![]
    }

    fn identifier_expr(&self, id: &str) -> String {
        ident(id)
    }

    fn primitive_expr(&self, primitive: Primitive) -> String {
        match primitive {
            Primitive::String => "String",
            Primitive::Bool => "Bool",
            Primitive::Number => "Double",
        }.to_string()
    }

    fn array_expr(&self, elem: &str) -> String {
        format!("[{elem}]")
    }

    fn optional_expr(&self, unwrapped: &str) -> String {
        format!("{unwrapped}?")
    }

    fn struct_decl(&self, struct_: Struct) -> Block {
        block![
            format!("public struct {}: {} {{", ident(&struct_.name), PROTOCOLS),
            block(struct_.fields.iter().map(|(name, ty)| format!("public var {}: {}", name, ty))),
            "}"
        ]
    }

    fn enum_decl(&self, enum_: Enum) -> Block {
        let value_type = match enum_.value_type {
            EnumValueType::String => "String",
            EnumValueType::Integer => "Int64",
        };
        block![
            format!("public enum {}: {}, {} {{", ident(&enum_.name), value_type, PROTOCOLS),
            block(enum_.values.into_iter().map(|(name, val)| format!("case {} = {}", ident(&name), match enum_.value_type {
                EnumValueType::Integer => val,
                EnumValueType::String => quote(&val),
            }))),
            "}"
        ]
    }

    fn union_decl(&self, _union: Union) -> Block {
        todo!()
    }
}

const KEYWORDS: &[&str] = &[
    // Keywords used in declarations
    "associatedtype",
    "class",
    "deinit",
    "enum",
    "extension",
    "fileprivate",
    "func",
    "import",
    "init",
    "inout",
    "internal",
    "let",
    "open",
    "operator",
    "private",
    "precedencegroup",
    "protocol",
    "public",
    "rethrows",
    "static",
    "struct",
    "subscript",
    "typealias",
    "var",
    //Keywords used in statements
    "break",
    "case",
    "catch",
    "continue",
    "default",
    "defer",
    "do",
    "else",
    "fallthrough",
    "for",
    "guard",
    "if",
    "in",
    "repeat",
    "return",
    "throw",
    "switch",
    "where",
    "while",
    // Keywords used in expressions and types
    "Any",
    "as",
    "catch",
    "false",
    "is",
    "nil",
    "rethrows",
    "self",
    "Self",
    "super",
    "throw",
    "throws",
    "true",
    "try",
    //Keywords used in patterns:
    "_",
];
