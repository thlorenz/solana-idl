use anchor_lang_idl::types::IdlField as NewIdlField;
use solana_idl_classic::IdlField;

use crate::errors::IdlConverterResult;

use super::idl_type;

pub fn try_convert(idl_field: NewIdlField, context: &str) -> IdlConverterResult<IdlField> {
    let NewIdlField { name, ty, docs } = idl_field;
    let ty = idl_type::try_convert(ty, &format!("{} Field: {}", context, &name))?;
    Ok(IdlField {
        name,
        ty,
        attrs: Some(docs),
    })
}
