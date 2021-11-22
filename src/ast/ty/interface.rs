use swc_ecma_ast::{TsTypeElement, Expr};
use indexmap::IndexMap;
use crate::ast::iter_hash::HashableIndexMap;
use super::{ParsingContext, Type};
use crate::Result;

impl<'a> ParsingContext<'a> {
    pub fn parse_type_elements(&self, type_elements: &[TsTypeElement]) -> Result<HashableIndexMap<String, Type>>{
        let props = type_elements.iter().map(|member| match member {
            TsTypeElement::TsPropertySignature(ts_prop_signature) => {
                let key = match ts_prop_signature.key.as_ref() {
                    Expr::Ident(ident) => ident.sym.to_string(),
                    other => return Err(self.unexpected_spanned(other)),
                };
                let mut ts_type = match ts_prop_signature.type_ann.as_ref() {
                    Some(type_ann) => self.parse_type(type_ann.type_ann.as_ref())?,
                    None => return Err(self.unexpected_spanned(ts_prop_signature)),
                };
                if ts_prop_signature.optional {
                    ts_type.make_optional();
                }
                Ok((key, ts_type))
            },
            _ => Err(self.unexpected_spanned(member))
        }).collect::<Result<IndexMap<String, Type>>>()?;
    }
}
