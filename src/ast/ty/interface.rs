use swc_ecma_ast::{TsTypeElement, Expr, TsInterfaceDecl};
use indexmap::IndexMap;
use crate::ast::iter_hash::HashableIndexMap;
use crate::ast::TypeDecl;
use super::{ParsingContext, Type};
use crate::Result;

impl<'a> ParsingContext<'a> {
    pub fn parse_type_elements(&self, type_elements: &[TsTypeElement]) -> Result<HashableIndexMap<String, Type>>{
        let props = type_elements
            .iter()
            .map(|elem| match elem {
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
            _ => Err(self.unexpected_spanned(elem))
        }).collect::<Result<IndexMap<String, Type>>>()?;
        Ok(props.into())
    }
    pub fn parse_interface_decl(&self, ts_interface_decl: &TsInterfaceDecl) -> Result<TypeDecl> {
        let name = ts_interface_decl.id.sym.to_string();
        let type_params = if let Some(some) = ts_interface_decl.type_params.as_ref() {
            self.get_type_params(some)?
        } else {
            vec![]
        };
        let interface_props = self.parse_type_elements(ts_interface_decl.body.body.as_slice())?;
        Ok(TypeDecl {
            name, type_params, ty: Type::Interface(interface_props)
        })
    }
}
