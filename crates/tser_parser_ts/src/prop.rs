use crate::error::StructureError;

use crate::type_expr::parse_to_type_expr;
use swc_common::Spanned;
use swc_ecma_ast::{Expr, TsPropertySignature, TsType, TsTypeElement};
use tser_ir::type_decl::struct_::Field;

pub struct Prop<'a> {
    pub name: String,
    pub optional: bool,
    pub ts_type: &'a TsType,
    pub ts_type_element: &'a TsTypeElement,
}

impl<'a> TryFrom<&'a Prop<'a>> for Field {
    type Error = StructureError;
    fn try_from(prop: &Prop) -> Result<Self, Self::Error> {
        Ok(Field {
            name: prop.name.clone(),
            optional: prop.optional,
            ty: parse_to_type_expr(prop.ts_type)?,
        })
    }
}

pub fn parse_as_prop(ts_type_element: &TsTypeElement) -> Result<Prop, StructureError> {
    match ts_type_element {
        TsTypeElement::TsPropertySignature(prop_sig) => match prop_sig {
            TsPropertySignature {
                key,
                params,
                init: None,
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
                    ts_type_element,
                })
            }
            other => Err(other.span.into()),
        },
        other => Err(other.span().into()),
    }
}
