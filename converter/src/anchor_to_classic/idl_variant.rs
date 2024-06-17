use anchor_lang_idl::types::IdlEnumVariant as NewIdlEnumVariant;
use solana_idl_classic::IdlEnumVariant;

use crate::errors::IdlConverterResult;

use super::idl_defined_fields;

pub fn try_convert(
    idl_variant: NewIdlEnumVariant,
    context: &str,
) -> IdlConverterResult<IdlEnumVariant> {
    let NewIdlEnumVariant { name, fields } = idl_variant;
    let fields = match fields {
        Some(field) => Some(idl_defined_fields::try_convert_to_enum(field, context)?),
        None => None,
    };
    Ok(IdlEnumVariant { name, fields })
}
