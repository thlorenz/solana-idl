use anchor_lang_idl::types::{
    IdlAccount as NewIdlAccount, IdlTypeDef as NewIdlTypeDef, IdlTypeDefTy as NewIdlTypeDefTy,
};
use solana_idl_classic::{IdlTypeDefinition, IdlTypeDefinitionTy};

use crate::{
    anchor_to_classic::{idl_defined_fields, idl_variant},
    errors::{IdlConverterError, IdlConverterResult},
};
pub fn try_convert_type_def_ty(
    idl_type_def_ty: NewIdlTypeDefTy,
    context: &str,
) -> IdlConverterResult<IdlTypeDefinitionTy> {
    use NewIdlTypeDefTy::*;
    let converted: IdlTypeDefinitionTy = match idl_type_def_ty {
        Struct { fields } => {
            let fields = match fields {
                Some(fields) => idl_defined_fields::try_convert_to_idl_fields(fields, context)?,
                None => vec![],
            };
            IdlTypeDefinitionTy::Struct { fields }
        }
        Enum { variants } => IdlTypeDefinitionTy::Enum {
            variants: variants
                .into_iter()
                .map(|variant| idl_variant::try_convert(variant, context))
                .collect::<IdlConverterResult<Vec<_>>>()?,
        },
        Type { alias: _ } => Err(IdlConverterError::UnsupportedClassicIdlType(
            "IdlTypeDefTy::Type".to_string(),
            context.to_string(),
        ))?,
    };
    Ok(converted)
}

pub fn try_convert_type_definition(
    idl_type_definition: NewIdlTypeDef,
) -> IdlConverterResult<IdlTypeDefinition> {
    let NewIdlTypeDef { name, ty, .. } = idl_type_definition;
    let ty = try_convert_type_def_ty(ty, &name)?;
    Ok(IdlTypeDefinition { name, ty })
}

pub fn try_convert_from_account(idl_account: NewIdlAccount) {}
