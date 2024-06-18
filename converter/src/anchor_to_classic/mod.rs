#[allow(unused_imports)]
use anchor_lang_idl::types::{
    Idl as NewIdl, IdlAccount as NewIdlAccount, IdlConst as NewIdlConst,
    IdlErrorCode as NewIdlErrorCode, IdlEvent as NewIdlEvent, IdlInstruction as NewIdlInstruction,
    IdlMetadata as NewIdlMetadata, IdlTypeDef as NewIdlTypeDef,
};
use solana_idl_classic::Idl;

use crate::errors::IdlConverterResult;
mod idl_defined_fields;
mod idl_error_code;
mod idl_field;
mod idl_instruction;
mod idl_metadata;
mod idl_type;
mod idl_type_definition;
mod idl_variant;

pub fn try_convert(idl: NewIdl) -> IdlConverterResult<Idl> {
    let NewIdl {
        address,
        metadata,
        docs: _,
        instructions,
        accounts: _,
        events: _,
        errors: _,
        types: _,
        constants: _,
    } = idl;

    let name = metadata.name.clone();
    let metadata = idl_metadata::convert(metadata, Some(address));
    let instructions = instructions
        .into_iter()
        .map(idl_instruction::try_convert)
        .collect::<IdlConverterResult<Vec<_>>>()?;

    // TODO: accounts now just point to a type definition
    // so we may need to replicate it here

    todo!()
}
