use tser_codegen::rust::RustCodeGen;
use tser_codegen::swift::SwiftCodeGen;
use tser_codegen::{generate, CodeGen};
use tser_parser_ts::parse_file;

#[derive(Debug, Copy, Clone)]
pub enum Language {
    Rust,
    Swift,
}

pub fn generate_from_ts(ts_src: &str, lang: Language) -> anyhow::Result<String> {
    let code_gen: Box<dyn CodeGen> = match lang {
        Language::Rust => Box::new(RustCodeGen),
        Language::Swift => Box::new(SwiftCodeGen),
    };
    let ir_file = parse_file(ts_src)?;
    Ok(generate(&ir_file, code_gen.as_ref()))
}
