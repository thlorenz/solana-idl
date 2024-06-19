use anchor_lang_idl::types::IdlConst as NewIdlConst;
use solana_idl_classic::IdlConst;

use super::idl_type;
use crate::errors::IdlConverterResult;

pub fn try_convert(idl_const: NewIdlConst) -> IdlConverterResult<IdlConst> {
    let NewIdlConst {
        name,
        value,
        docs: _,
        ty,
    } = idl_const;
    let ty = idl_type::try_convert(ty, &name)?;
    Ok(IdlConst { name, ty, value })
}
