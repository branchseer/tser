#[cfg(test)]
mod tests {
    use swc_common::{FileName, SourceMap};
    use swc_common::sync::Lrc;
    use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax, TsConfig};

    #[test]
    fn it_works() {
        let cm: Lrc<SourceMap> = Default::default();

        let fm = cm.new_source_file(
            FileName::Custom("[tser-input].ts".into()),
            "export interface Hello { world: string }".into(),
        );
        let mut parser = Parser::new(Syntax::Typescript(TsConfig::default()), StringInput::from(&*fm), None);
        let module = parser.parse_module().unwrap();
        for item in module.body {
            match item {

            }
            println!("{:?}", item);
        }
    }
}
