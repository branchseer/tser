use super::{ConstantDecl, Lit, ParsingContext};
use crate::Result;
use swc_ecma_ast::{BindingIdent, Expr, Lit as SwcLit, Pat, VarDecl, VarDeclarator};

impl<'a> ParsingContext<'a> {
    pub fn parse_lit(&self, swc_lit: &SwcLit) -> Result<Lit> {
        Ok(match swc_lit {
            SwcLit::Str(str) => Lit::String(str.value.to_string()),
            SwcLit::Bool(bool) => Lit::Boolean(bool.value),
            SwcLit::Null(_) => Lit::Null,
            SwcLit::Num(num) => Lit::Number(num.value.try_into().unwrap()),
            _ => return Err(self.unexpected_spanned(swc_lit)),
        })
    }

    pub fn parse_var_declarator(&self, var_declarator: VarDeclarator) -> Result<ConstantDecl> {
        let var_name = match &var_declarator.name {
            Pat::Ident(BindingIdent { id, .. }) => id.sym.to_string(),
            _ => return Err(self.unexpected_ast_node(&var_declarator)),
        };
        let lit = match &var_declarator.init {
            Some(val_expr) => {
                let val_expr = val_expr.as_ref();
                match val_expr {
                    Expr::Lit(lit) => self.parse_lit(lit)?,
                    _ => return Err(self.unexpected_spanned(val_expr)),
                }
            }
            None => return Err(self.unexpected_ast_node(&var_declarator)),
        };
        Ok(ConstantDecl {
            name: var_name,
            value: lit,
        })
    }

    pub fn parse_val_decl(&self, var_decl: VarDecl) -> Result<Vec<ConstantDecl>> {
        var_decl
            .decls
            .into_iter()
            .map(|var_declarator| self.parse_var_declarator(var_declarator))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::super::{parse, ConstantDecl, Lit, Module, ModuleItem};
    #[test]
    fn test_val_decl() {
        let module = parse(
            Default::default(),
            r#"
        export const NUM = 4.2, STR = 'hello';
        export const TRUE = true;"#,
        )
        .unwrap();
        assert_eq!(
            module,
            Module {
                items: vec![
                    ModuleItem::Constant(ConstantDecl {
                        name: "NUM".to_string(),
                        value: Lit::Number((4.2).try_into().unwrap())
                    }),
                    ModuleItem::Constant(ConstantDecl {
                        name: "STR".to_string(),
                        value: Lit::String("hello".to_string())
                    }),
                    ModuleItem::Constant(ConstantDecl {
                        name: "TRUE".to_string(),
                        value: Lit::Boolean(true)
                    })
                ]
            }
        );
    }
}
