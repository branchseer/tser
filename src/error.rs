use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use swc_common::{Span, Spanned, AstNode};

#[derive(Debug)]
pub(crate) struct SourcePos {
    pub line: u32,
    pub col: u32
}

impl Display for SourcePos {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("line {}, column {}", self.line + 1, self.col + 1))
    }
}

#[derive(Debug)]
pub(crate) enum Kind {
    StructureNotSupported(Option<&'static str>),
    SwcParseError(swc_ecma_parser::error::SyntaxError),
    DefaultTypeParamNotSupported(String),
}

#[derive(Debug)]
pub struct Error {
    pub(crate) pos: SourcePos,
    pub(crate) kind: Kind
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f) // TODO: friendly error message
    }
}

impl std::error::Error for Error { }
