mod error;
mod prop;
mod type_decl;
mod type_expr;

#[cfg(test)]
mod test_utils;

use error::StructureError;
use std::fmt::{Display, Formatter};
use swc_common::input::StringInput;
use tser_ir::{File, Item};

use swc_common::{BytePos, Span, Spanned};
use swc_ecma_ast::{ExportDecl, Module, ModuleDecl, ModuleItem, Stmt};
use swc_ecma_parser::lexer::Lexer;
use swc_ecma_parser::{Parser, Syntax, TsConfig};
use type_decl::parse_type_decl;

fn parse_module(module: &Module) -> Result<File, StructureError> {
    let items = module
        .body
        .iter()
        .map(parse_module_item)
        .collect::<Result<Vec<Item>, StructureError>>()?;
    Ok(File { items })
}
fn parse_module_item(item: &ModuleItem) -> Result<Item, StructureError> {
    let decl = match item {
        ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(ExportDecl { decl, .. })) => decl,
        ModuleItem::Stmt(Stmt::Decl(decl)) => decl,
        other => return Err(other.span().into()),
    };
    let type_decl = parse_type_decl(decl)?;
    Ok(Item::TypeDecl(type_decl))
}

#[derive(Debug)]
pub(crate) struct SourcePos {
    pub line: u32,
    pub col: u32,
}
impl SourcePos {
    fn from_source_span(source: &str, span: Span) -> Self {
        let idx = span.lo.0;
        let mut col = idx;
        let mut line = 0_u32;
        for line_len in source.lines().map(|s| s.len() as u32) {
            if col > line_len {
                col -= line_len;
                line += 1;
            } else {
                break;
            }
        }
        SourcePos { line, col }
    }
}

impl Display for SourcePos {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Line {}, Column {}", self.line, self.col))
    }
}

fn ts_parser_from_source<'a>(source: &'a str) -> Parser<Lexer<StringInput<'a>>> {
    Parser::new(
        Syntax::Typescript(TsConfig::default()),
        StringInput::new(source, BytePos(0), BytePos(source.len() as u32)),
        None,
    )
}

// const ASYNC_ITERABLE: &str = "AsyncIterable";
const UNION_DISCRIMINATION_FIELD: &str = "type";

pub fn parse_file(source: &str) -> anyhow::Result<File> {
    let mut parser = ts_parser_from_source(source);
    let module = match parser.parse_module() {
        Ok(ok) => ok,
        Err(parser_error) => anyhow::bail!(
            "{} {}",
            SourcePos::from_source_span(source, parser_error.span()),
            parser_error.kind().msg()
        ),
    };
    let file = match parse_module(&module) {
        Ok(ok) => ok,
        Err(structure_error) => anyhow::bail!(
            "{} {}",
            SourcePos::from_source_span(source, structure_error.span),
            structure_error
                .message
                .unwrap_or_else(|| "Unrecognized structure".to_string())
        ),
    };
    Ok(file)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_add() {}
}
