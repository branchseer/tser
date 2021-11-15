mod constant;

pub enum BasicType {
    String,
}

pub enum ModuleItem {
    Submodule { name: String, module: Module },
}

pub struct Module {
    items: Vec<ModuleItem>,
}

const CRATE_NAME: &str = std::module_path!();

pub fn module_path() -> &'static str {
    std::module_path!()
}

#[cfg(test)]
mod tests {
    use swc_common::sync::Lrc;
    use swc_common::{FileName, SourceMap};
    use swc_ecma_ast::{Decl, ExportDecl, ModuleDecl, ModuleItem};
    use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax, TsConfig};

    #[test]
    fn hello_serde() {
        use serde::{Serialize, Deserialize};

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(tag="type", rename="1")]
        struct MyStruct1 {
            a: String
        }
        #[derive(Debug, Serialize, Deserialize)]
        #[serde(tag="type", rename="2")]
        struct MyStruct2 {
            a: String
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(untagged)]
        enum SumType {
            M1(MyStruct1),
            M2(MyStruct2)
        }


        println!("{:?}", serde_json::from_str::<SumType>(r#"{ "type": "2", "a": "aa" }"#));
    }
    

    #[test]
    fn it_works() {
        println!("{:?}", crate::CRATE_NAME);
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
            match &item {
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
