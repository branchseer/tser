use swc_ecma_ast::TsLit;
use super::{Lit, ParsingContext};
use crate::Result;

impl<'a> ParsingContext<'a> {
    pub fn parse_ts_lit(&self, ts_lit: &TsLit) -> Result<Lit> {
        Ok(match ts_lit {
            TsLit::Number(number) => Lit::Number(number.value.try_into().unwrap()),
            TsLit::Str(string) => Lit::String(string.value.to_string()),
            TsLit::Bool(bool) => Lit::Boolean(bool.value),
            _ => return Err(self.unexpected_spanned(ts_lit))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::super::Lit;
    use crate::ast::{Module, ModuleItem, parse, Type, TypeDecl};

    #[test]
    fn test_lit_type() {
        let module = parse(Default::default(), r#"
    export type True = true;
    export type Hello = 'hello';
    export type Null = null;
    export type TheAnswer = 42;
    "#).unwrap();
        assert_eq!(module, Module {
            items: vec![
                ModuleItem::Type(TypeDecl {
                    name: "True".to_string(),
                    type_params: vec![],
                    ty: Type::Lit(Lit::Boolean(true))
                }),
                ModuleItem::Type(TypeDecl {
                    name: "Hello".to_string(),
                    type_params: vec![],
                    ty: Type::Lit(Lit::String("hello".to_string()))
                }),
                ModuleItem::Type(TypeDecl {
                    name: "Null".to_string(),
                    type_params: vec![],
                    ty: Type::Lit(Lit::Null)
                }),
                ModuleItem::Type(TypeDecl {
                    name: "TheAnswer".to_string(),
                    type_params: vec![],
                    ty: Type::Lit(Lit::Number(42.try_into().unwrap()))
                })
            ]
        });
    }
}

