use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use swc_common::{Span, Spanned, AstNode};

#[derive(Debug)]
pub enum Error {
    StructureNotSupported {
        name: &'static str,
        line: u32,
        col: u32,
    },
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::StructureNotSupported { name, line, col } => f.write_fmt(format_args!(
                "structure not supported: {} at line {}, column {}",
                *name, *line, *col
            )),
        }
    }
}

pub(crate) struct ErrorFactory<'a> {
    source: &'a str
}

impl ErrorFactory {
    pub fn structure_not_supported<S: AstNode>(&self, s: &S) -> Error {
        let idx = s.span().lo.0;
        let mut col = idx;
        let mut line = 0_u32;
        for line_len in self.source.lines().map(|s|s.len() as u32) {
            if col > line_len {
                col -= line_len;
                line += 1;
            }
            else {
                break
            }
        }
        Error::StructureNotSupported {
            name: S::TYPE,
            line, col
        }
    }
}