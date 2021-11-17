mod lit;

enum BasicType {
    String,
    Number,
    Boolean,
    Null,
    Undefined
}

struct TypeRef {
    module_path: Vec<String>,
    type_name: String,
}

enum Type {
    Basic(BasicType),
    Lit(Lit),
    Ref(TypeRef),
    Union(Vec<Type>),
    Tuple(Vec<Type>),
}

enum Lit {
    String(String),
    Number(f64),
    Boolean(bool),
    Nullish, // null or undefined, we don't distinguish them
}

enum ModuleItem {
    Submodule { name: String, module: Module },
    ConstValue { name: String, value: Lit },
    Type { name: String, ty: Type }
}

pub struct Module {
    items: Vec<ModuleItem>,
}

pub(crate) const CRATE_NAME: &str = env!("CARGO_CRATE_NAME");

#[cfg(test)]
mod tests {
    use swc_common::sync::Lrc;
    use swc_common::{FileName, SourceMap};
    use swc_ecma_ast::{Decl, ExportDecl, ModuleDecl, ModuleItem};
    use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax, TsConfig};

    #[test]
    fn it_works() {
        println!("{:?}", crate::CRATE_NAME);
        let cm: Lrc<SourceMap> = Default::default();

        let fm = cm.new_source_file(
            FileName::Custom("[tser-input].ts".into()),
            "namespace X { export interface A {} } ; export type B = X.Y.A;".into(),
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
