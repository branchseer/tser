use crate::{CodeGen, Enum, EnumValueType, Struct, Union};
use tser_block::{block, Block};
use tser_ir::type_expr::primitive::Primitive;

pub struct RustCodeGen;

const DERIVE_LINE: &str = "#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]";

fn quote(string: &str) -> String {
    format!("\"{}\"", string.escape_default())
}
fn ident(id: &str) -> String {
    id.to_string() // TODO: check keywords
}

impl CodeGen for RustCodeGen {
    fn head(&self) -> Block {
        block!["use serde::{Serialize, Deserialize};", ""]
    }
    fn identifier_expr(&self, id: &str) -> String {
        ident(id)
    }
    fn primitive_expr(&self, primitive: Primitive) -> String {
        match primitive {
            Primitive::String => "String",
            Primitive::Number => "f64",
            Primitive::Bool => "bool",
        }
        .to_string()
    }
    fn array_expr(&self, elem: &str) -> String {
        format!("Vec<{}>", elem)
    }
    fn optional_expr(&self, unwrapped: &str) -> String {
        format!("Option<{}>", unwrapped)
    }
    fn struct_decl(&self, struct_: Struct) -> Block {
        block![
            DERIVE_LINE,
            format!("pub struct {} {{", struct_.name),
            block(
                struct_
                    .fields
                    .into_iter()
                    .map(|(field, ty)| format!("pub {field}: {ty},"))
            ),
            "}",
        ]
    }
    fn enum_decl(&self, enum_: Enum) -> Block {
        block![
            DERIVE_LINE,
            match enum_.value_type {
                EnumValueType::Integer => Some("#[repr(i64)]"),
                EnumValueType::String => None,
            },
            format!("pub enum {} {{", enum_.name),
            block(
                enum_
                    .values
                    .into_iter()
                    .map(|(name, val)| match enum_.value_type {
                        EnumValueType::Integer => format!("{} = {},", ident(&name), val),
                        EnumValueType::String =>
                            format!("#[serde(rename = {})] {},", quote(&val), ident(&name)),
                    })
            ),
            "}"
        ]
    }
    fn union_decl(&self, _union: Union) -> Block {
        unimplemented!()
    }
}
