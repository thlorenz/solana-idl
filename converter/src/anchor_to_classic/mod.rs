#[allow(unused_imports)]
use anchor_lang_idl::types::{
    Idl as NewIdl, IdlAccount as NewIdlAccount, IdlConst as NewIdlConst,
    IdlErrorCode as NewIdlErrorCode, IdlEvent as NewIdlEvent, IdlInstruction as NewIdlInstruction,
    IdlMetadata as NewIdlMetadata, IdlTypeDef as NewIdlTypeDef,
};
use solana_idl_classic::Idl;

use crate::errors::IdlConverterResult;

mod idl_const;
mod idl_defined_fields;
mod idl_error_code;
mod idl_event;
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
        accounts,
        events,
        errors,
        types,
        constants,
    } = idl;

    let name = metadata.name.clone();
    let version = metadata.version.clone();
    let metadata = idl_metadata::convert(metadata, Some(address));
    let constants = constants
        .into_iter()
        .map(idl_const::try_convert)
        .collect::<IdlConverterResult<Vec<_>>>()?;
    let instructions = instructions
        .into_iter()
        .map(idl_instruction::try_convert)
        .collect::<IdlConverterResult<Vec<_>>>()?;
    let types = types
        .into_iter()
        .map(idl_type_definition::try_convert_type_definition)
        .collect::<IdlConverterResult<Vec<_>>>()?;
    let errors = if errors.is_empty() {
        None
    } else {
        Some(
            errors
                .into_iter()
                .map(idl_error_code::convert)
                .collect::<Vec<_>>(),
        )
    };
    let accounts = accounts
        .into_iter()
        .map(idl_type_definition::convert_from_account)
        .collect::<Vec<_>>();

    let events = if events.is_empty() {
        None
    } else {
        Some(
            events
                .into_iter()
                .map(idl_event::convert)
                .collect::<Vec<_>>(),
        )
    };

    Ok(Idl {
        version,
        name,
        constants,
        instructions,
        types,
        errors,
        metadata: Some(metadata),
        // No equivalent in new IDL format
        state: None,
        accounts,
        events,
    })
}
