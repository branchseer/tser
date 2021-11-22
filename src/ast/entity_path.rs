use swc_ecma_ast::{TsEntityName, TsQualifiedName};
use super::{ParsingContext, EntityPath};
use crate::Result;

impl<'a> ParsingContext<'a> {

    fn flatten_qualified_name(&self, ts_qualified_name: &TsQualifiedName) -> Result<EntityPath> {
        let left_path = self.parse_entity_path(&ts_qualified_name.left)?;
        let mut ancestors = left_path.ancestors;
        ancestors.push(left_path.name);
        Ok(EntityPath {
            ancestors,
            name: ts_qualified_name.right.sym.to_string(),
        })
    }

    pub fn parse_entity_path(&self, ts_entity_name: &TsEntityName) -> Result<EntityPath> {
        Ok(match ts_entity_name {
            TsEntityName::Ident(ident) => EntityPath {
                ancestors: vec![],
                name: ident.sym.to_string(),
            },
            TsEntityName::TsQualifiedName(qualified_name) => {
                self.flatten_qualified_name(qualified_name)?
            }
        })
    }
}


#[cfg(test)]
mod tests {
    use super::super::{parse, Module, ModuleItem, TypeDecl, Type, TypeRef, EntityPath};

    #[test]
    fn test_entity_path() {
        let module = parse(Default::default(), "export type X = A.B.C").unwrap();
        assert_eq!(module, Module { items: vec![
            ModuleItem::Type(TypeDecl {
                name: "X".to_string(),
                type_params: vec![],
                ty: Type::Ref(TypeRef {
                    entity_path: EntityPath {
                        ancestors: vec!["A".to_string(), "B".to_string()],
                        name: "C".to_string()
                    },
                    generic_args: vec![]
                }) })
        ] });
    }
}