use anchor_lang_idl::types::Idl as NewIdl;
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

/// Does a best attempt at converting a new anchor>=0.30.0 IDL to the classic format
/// that tools like solita, chainparser and most explorers understand.
/// Some cases are not supported and will return an error, i.e. if data types like
/// `u256` are used that weren't supported in the classic IDL format.
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
        .map(|acc| idl_type_definition::convert_from_account(acc, &types))
        .collect::<Vec<_>>();

    let events = if events.is_empty() {
        None
    } else {
        Some(
            events
                .into_iter()
                .map(|ev| idl_event::convert(ev, &types))
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
