pub enum BasicType {
    String,
}

enum Type {

}

pub enum ModuleItem {
    Submodule { name: String, module: Module },
}

pub struct Module {
    items: Vec<ModuleItem>,
}

#[cfg(test)]
mod tests {
    use swc_common::sync::Lrc;
    use swc_common::{FileName, SourceMap};
    use swc_ecma_ast::{Decl, ExportDecl, ModuleDecl, ModuleItem};
    use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax, TsConfig};

    #[test]
    fn it_works() {
        let cm: Lrc<SourceMap> = Default::default();

        let fm = cm.new_source_file(
            FileName::Custom("[tser-input].ts".into()),
            "export interface Hello { world: string } namespace Ahaha {}".into(),
        );
        let mut parser = Parser::new(
            Syntax::Typescript(TsConfig::default()),
            StringInput::from(&*fm),
            None,
        );
        let module = parser.parse_module().unwrap();

        for item in module.body {
            match item {
                ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(ExportDecl { decl, .. })) => {
                    match decl {
                        Decl::Class(_) => {}
                        Decl::Fn(_) => {}
                        Decl::Var(_) => {}
                        Decl::TsInterface(_) => {}
                        Decl::TsTypeAlias(_) => {}
                        Decl::TsEnum(_) => {}
                        Decl::TsModule(_) => {}
                    }
                }
                other => {
                    eprintln!("Unexpected structure")
                }
            }
            println!("{:?}", item);
        }
    }
}
