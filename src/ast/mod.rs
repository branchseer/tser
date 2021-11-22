mod module;
mod lit;
mod ty;
mod entity_path;
mod iter_hash;

use std::borrow::BorrowMut;
use std::path::PathBuf;
use std::str::FromStr;
use indexmap::{IndexMap, IndexSet, indexset};
use swc_common::{AstNode, Span, Spanned};
use rust_decimal::Decimal;
use swc_ecma_ast::{TsTypeParam};
use crate::error::{SourcePos, Kind};
use crate::{Error, Result};
use crate::ast::iter_hash::{HashableIndexMap, HashableIndexSet};

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum BasicType {
    String,
    Number,
    Boolean,
}


/// Represents A.B.C (ancestors: ["A", "B"], name: "C")
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct EntityPath {
    ancestors: Vec<String>,
    name: String,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct TypeRef {
    entity_path: EntityPath,
    generic_args: Vec<Type>,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum EnumType {
    Strings(HashableIndexMap<String, String>),
    Numbers(HashableIndexMap<String, isize>),
}


#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum Type {
    Basic(BasicType),
    Lit(Lit),
    Ref(TypeRef),
    Union(HashableIndexSet<Type>),
    Tuple(Vec<Type>),
    Enum(EnumType),
    Array(Box<Type>),
    Map(Box<Type>),
    Interface(HashableIndexMap<String, Type>),
    Unspecified // any or null
}

impl Type {
    fn union_add(&mut self, ty: Self) -> bool {
        if let Type::Union(union_elems) = self {
            union_elems.as_mut().insert(ty)
        }
        else {
            let original_self = std::mem::replace(self, Type::Unspecified);
            *self = Type::Union(indexset! { original_self, ty }.into());
            true
        }
    }
    fn union_remove(&mut self, ty: &Self) -> bool {
        if let Type::Union(union_elems) = self {
            let union_elems: &mut IndexSet<Type> = union_elems.as_mut();
            if union_elems.remove(ty) {
                if union_elems.len() == 1 {
                    let single_elem = union_elems.pop().unwrap();
                    *self = single_elem
                }
                return true
            }
        }
        false
    }
    pub fn make_optional(&mut self) -> bool {
        self.union_add(Type::Lit(Lit::Null))
    }
    pub fn make_non_optional(&mut self) -> bool {
        self.union_remove(&Type::Lit(Lit::Null))
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum Lit {
    String(String),
    Number(Decimal),
    Boolean(bool),
    Null // null or undefined or optional type
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct ConstantDecl {
    name: String,
    value: Lit,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct TypeDecl {
    name: String,
    type_params: Vec<String>,
    ty: Type,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum ModuleItem {
    Submodule(Submodule),
    Constant(ConstantDecl),
    Type(TypeDecl),
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Submodule {
    pub module_name: String,
    pub module: Module,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Module {
    pub items: Vec<ModuleItem>,
}

struct ParsingContext<'a> {
    source: &'a str
}


impl<'a> ParsingContext<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { source }
    }
    fn pos_from_span(&self, s: Span) -> SourcePos {
        let idx = s.span().lo.0;
        let mut col = idx;
        let mut line = 0_u32;
        for line_len in self.source.lines().map(|s| s.len() as u32) {
            if col > line_len {
                col -= line_len;
                line += 1;
            } else {
                break
            }
        }
        SourcePos { line, col }
    }
    pub fn unexpected_spanned<S: Spanned>(&self, s: &S) -> Error {
        Error {
            pos: self.pos_from_span(s.span()),
            kind: Kind::StructureNotSupported(None),
        }
    }
    pub fn unexpected_ast_node<S: AstNode>(&self, s: &S) -> Error {
        Error {
            pos: self.pos_from_span(s.span()),
            kind: Kind::StructureNotSupported(Some(S::TYPE)),
        }
    }
    pub fn swc_parse_error(&self, swc_error: swc_ecma_parser::error::Error) -> Error {
        Error {
            pos: self.pos_from_span(swc_error.span()),
            kind: Kind::SwcParseError(swc_error.into_kind())
        }
    }
    pub fn default_type_param_error(&self, type_param: &TsTypeParam) -> Error {
        Error {
            pos: self.pos_from_span(type_param.span),
            kind: Kind::DefaultTypeParamNotSupported(type_param.name.sym.to_string())
        }
    }
}


pub(crate) fn parse(filename: PathBuf, source: &str) -> Result<Module> {
    use swc_common::sync::Lrc;
    use swc_common::{FileName, SourceMap};
    use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax, TsConfig};

    let cm: Lrc<SourceMap> = Default::default();

    let fm = cm.new_source_file(
        FileName::Real(filename),
        source.into(),
    );
    let mut parser = Parser::new(
        Syntax::Typescript(TsConfig::default()),
        StringInput::from(&*fm),
        None,
    );
    let ctx = ParsingContext::new(source);
    let module = parser.parse_module().map_err(|err|ctx.swc_parse_error(err))?;
    ctx.parse_top_level_module(module)
}
