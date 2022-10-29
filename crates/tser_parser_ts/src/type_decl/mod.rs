mod enum_;
mod struct_;
mod union;

use crate::error::StructureError;
use crate::type_decl::enum_::parse_enum;
use crate::type_decl::struct_::parse_struct;
use crate::type_decl::union::parse_union;
use swc_common::Spanned;
use swc_ecma_ast::Decl;
use tser_ir::type_decl::TypeDecl;

pub fn parse_type_decl(decl: &Decl) -> Result<TypeDecl, StructureError> {
    Ok(match decl {
        Decl::TsInterface(ts_interface) => TypeDecl::Struct(parse_struct(ts_interface)?),
        Decl::TsEnum(ts_enum) => TypeDecl::Enum(parse_enum(ts_enum)?),
        Decl::TsTypeAlias(ts_type_alias) => TypeDecl::Union(parse_union(ts_type_alias)?),
        other => return Err(other.span().into()),
    })
}
