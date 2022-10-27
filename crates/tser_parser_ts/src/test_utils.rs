use crate::ts_parser_from_source;
use assert_matches::assert_matches;
use swc_ecma_ast::{Decl, Stmt};

pub fn parse_src_as_decl(src: &str) -> Decl {
    let mut parser = ts_parser_from_source(src);
    let stmt = parser.parse_stmt(true).unwrap();
    assert_matches!(stmt, Stmt::Decl(decl) => decl)
}
