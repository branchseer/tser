mod ast;
mod serde_lit;
mod swc_utils;
mod error;

pub use error::Error;
pub type Result<T> = std::result::Result<T, Error>;


#[test]
#[ignore]
fn swc_playground() {
    use swc_common::sync::Lrc;
    use swc_common::{FileName, SourceMap};
    use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax, TsConfig};

    let cm: Lrc<SourceMap> = Default::default();

    let fm = cm.new_source_file(
        FileName::Custom("".to_string()),
        "export type A = { a: null }".into(),
    );
    let mut parser = Parser::new(
        Syntax::Typescript(TsConfig::default()),
        StringInput::from(&*fm),
        None,
    );
    let module_res = parser.parse_module();
    dbg!(module_res);
}
