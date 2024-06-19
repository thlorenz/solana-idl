use anchor_lang_idl::types::IdlDefinedFields;
use solana_idl_classic::EnumFields;

use super::{idl_field, idl_type};
use crate::errors::IdlConverterResult;

pub fn try_convert_to_enum(
    idl_fields: IdlDefinedFields,
    context: &str,
) -> IdlConverterResult<EnumFields> {
    let fields = match idl_fields {
        IdlDefinedFields::Named(named_fields) => {
            let named_fields = named_fields
                .into_iter()
                .map(|field| idl_field::try_convert(field, context))
                .collect::<IdlConverterResult<Vec<_>>>()?;
            EnumFields::Named(named_fields)
        }
        IdlDefinedFields::Tuple(types) => {
            let types = types
                .into_iter()
                .map(|ty| idl_type::try_convert(ty, context))
                .collect::<IdlConverterResult<Vec<_>>>()?;
            EnumFields::Tuple(types)
        }
    };
    Ok(fields)
}

pub fn try_convert_to_idl_fields(
    idl_fields: IdlDefinedFields,
    context: &str,
) -> IdlConverterResult<Vec<solana_idl_classic::IdlField>> {
    match idl_fields {
        IdlDefinedFields::Named(named_fields) => named_fields
            .into_iter()
            .map(|field| idl_field::try_convert(field, context))
            .collect::<IdlConverterResult<Vec<_>>>(),
        IdlDefinedFields::Tuple(types) => types
            .into_iter()
            .enumerate()
            .map(|(idx, ty)| (idx, idl_type::try_convert(ty, context)))
            .map(|(idx, ty)| {
                Ok(solana_idl_classic::IdlField {
                    name: idx.to_string(),
                    ty: ty?,
                    attrs: None,
                })
            })
            .collect::<IdlConverterResult<Vec<_>>>(),
    }
}
