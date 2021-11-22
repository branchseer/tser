use swc_ecma_ast::{Decl, Stmt, Module as SwcModule, ModuleDecl, ModuleItem as SwcModuleItem, TsModuleBlock, TsModuleDecl, TsModuleName, TsNamespaceBody};
use itertools::Itertools;
use crate::ast::{Module, ModuleItem, Submodule};
use super::ParsingContext;

impl<'a> ParsingContext<'a> {
    fn parse_ts_sub_module(&self, ts_module_decl: TsModuleDecl) -> crate::Result<Submodule> {
        let module_name = match &ts_module_decl.id {
            TsModuleName::Str(str) => return Err(self.unexpected_spanned(&str)),
            TsModuleName::Ident(ident) => ident.sym.to_string(),
        };
        let mut swc_module_items = match ts_module_decl.body {
            Some(TsNamespaceBody::TsModuleBlock(TsModuleBlock { body, .. })) => {
                body
            },
            _ => return Err(self.unexpected_ast_node(&ts_module_decl)),
        };
        let items = swc_module_items
            .into_iter()
            .map(|item| self.parse_ts_module_item(item))
            .flatten_ok()
            .collect::<crate::Result<Vec<ModuleItem>>>()?;
        Ok(Submodule {
            module_name,
            module: Module {
                items
            }
        })
    }

    // TODO: use smallvec in return type, as we usually return one ModuleItem
    fn parse_ts_module_item(&self, swc_module_item: SwcModuleItem) -> crate::Result<Vec<ModuleItem>> {
        match swc_module_item {
            SwcModuleItem::ModuleDecl(ModuleDecl::ExportDecl(export_decl)) => {
                match export_decl.decl {
                    Decl::Var(var_decl) => {
                        let mut constant_decl = self.parse_val_decl(var_decl)?;
                        Ok(constant_decl.into_iter().map(ModuleItem::Constant).collect())
                    },
                    Decl::TsTypeAlias(type_alias_decl) => {
                        let type_decl = self.parse_type_alias(type_alias_decl)?;
                        Ok(vec![ModuleItem::Type(type_decl)])
                    },
                    Decl::TsInterface(ts_interface_decl) => {
                        let interface_decl = self.parse_interface_decl(&ts_interface_decl)?;
                        Ok(vec![ModuleItem::Type(interface_decl)])
                    },
                    _ => Err(self.unexpected_ast_node(&export_decl)),
                }
            },
            SwcModuleItem::Stmt(Stmt::Decl(Decl::TsModule(ts_module_decl))) => {
                let submodule = self.parse_ts_sub_module(ts_module_decl)?;
                Ok(vec![ModuleItem::Submodule(submodule)])
            },
            other => Err(self.unexpected_spanned(&other)),
        }
    }

    pub fn parse_top_level_module(&self, swc_module: SwcModule) -> crate::Result<Module> {
        let items = swc_module.body
            .into_iter()
            .map(|item| self.parse_ts_module_item(item))
            .flatten_ok()
            .collect::<crate::Result<Vec<ModuleItem>>>()?;
        Ok(Module {
            items
        })
    }
}



#[cfg(test)]
mod tests {
    use crate::ast::Submodule;
    use super::super::{parse, ConstantDecl, Lit, Module, ModuleItem};
    #[test]
    fn test_module() {
        let module = parse(
            Default::default(),
            r#"
        export const Hello = 0;
        namespace Hello {
            module NestedModule {
                export const foo = 'bar';
            }
            export const World = 1;
        }
        "#)
            .unwrap();
        assert_eq!(
            module,
            Module {
                items: vec![
                    ModuleItem::Constant(ConstantDecl {
                        name: "Hello".to_string(),
                        value: Lit::Number((0).try_into().unwrap())
                    }),
                    ModuleItem::Submodule(Submodule {
                        module_name: "Hello".to_string(),
                        module: Module { items: vec![
                            ModuleItem::Submodule(Submodule {
                                module_name: "NestedModule".to_string(),
                                module: Module { items: vec![
                                    ModuleItem::Constant(ConstantDecl {
                                        name: "foo".to_string(),
                                        value: Lit::String("bar".to_string())
                                    }),
                                ]}
                            }),
                            ModuleItem::Constant(ConstantDecl {
                                name: "World".to_string(),
                                value: Lit::Number("1".parse().unwrap())
                            }),
                        ] }
                    }),
                ]
            }
        );
    }
}
