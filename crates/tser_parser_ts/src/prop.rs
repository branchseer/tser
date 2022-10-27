use crate::error::StructureError;

use swc_common::Spanned;
use swc_ecma_ast::{Expr, TsPropertySignature, TsType, TsTypeElement};

pub struct Prop<'a> {
    pub name: String,
    pub optional: bool,
    pub ts_type: &'a TsType,
}

pub fn parse_as_prop(ts_type_element: &TsTypeElement) -> Result<Prop, StructureError> {
    match ts_type_element {
        TsTypeElement::TsPropertySignature(prop_sig) => match prop_sig {
            TsPropertySignature {
                key,
                params,
                init: Option::None,
                computed: false,
                optional,
                type_ann: Some(type_ann),
                ..
            } if params.is_empty() => {
                let name = match key.as_ref() {
                    Expr::Ident(ident) => ident.sym.to_string(),
                    other => return Err(other.span().into()),
                };
                Ok(Prop {
                    name,
                    optional: *optional,
                    ts_type: type_ann.type_ann.as_ref(),
                })
            }
            other => Err(other.span.into()),
        },
        other => Err(other.span().into()),
    }
}
