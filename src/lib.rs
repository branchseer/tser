mod lit;

use indexmap::IndexMap;
use swc_common::{Span};
use swc_ecma_ast::{
    ModuleItem as SwcModuleItem, ModuleDecl, ExportDecl, Decl, TsModuleDecl, TsModuleName,
    Str
};

#[derive(Debug)]
enum BasicType {
    String,
    Number,
    Boolean,
    Null,
    Undefined
}

#[derive(Debug)]
struct TypeRef {
    module_path: Vec<String>,
    type_name: String,
}

#[derive(Debug)]
enum EnumType {
    Strings(IndexMap<String, String>),
    Numbers(IndexMap<String, isize>),
}

#[derive(Debug)]
enum NonNullType {
    Basic(BasicType),
    Lit(Lit),
    Ref(TypeRef),
    Union(Vec<Type>),
    Tuple(Vec<Type>),
    Enum(EnumType)
}

#[derive(Debug)]
enum Type {
    NonNull(NonNullType),
    Nullish // null or undefined
}

#[derive(Debug)]
enum Lit {
    String(String),
    Number(f64),
    Boolean(bool),
}

#[derive(Debug)]
enum ModuleItem {
    Submodule { name: String, module: Module },
    ConstValue { name: String, value: Lit },
    Type { name: String, ty: Type }
}

#[derive(Debug)]
pub struct Module {
    items: Vec<ModuleItem>,
}

pub enum Error {
    StructureNotSupported { begin: u32, end: u32 }
}

impl Error {
    pub(crate) fn structure_not_supported(span: Span) -> Self {
        Self::StructureNotSupported {
            begin: span.lo.0, end: span.hi.0
        }
    }
}


pub type Result<T> = std::result::Result<T, Error>;

fn parse_ts_module_item(swc_module_item: SwcModuleItem) -> Result<ModuleItem> {
    match swc_module_item {
        SwcModuleItem::ModuleDecl(ModuleDecl::ExportDecl(ExportDecl { decl, span: export_decl_span })) => {
            match decl {
                Decl::TsModule(ts_module_decl) => {
                    match ts_module_decl.id {
                        TsModuleName::Str(Str { span, ..}) => return Err(Error::structure_not_supported(span))
                        TsModuleName::Ident(ident) => {}
                    }
                }
            }
        }
    }
    todo!()
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
            "namespace XZZZ { export interface A {} } ; export type B = X.Y.A;".into(),
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
