use anchor_lang_idl::types::IdlEvent as NewIdlEvent;
use solana_idl_classic::{IdlEvent, IdlEventField, IdlType};

pub fn convert(idl_event: NewIdlEvent) -> IdlEvent {
    let NewIdlEvent {
        name,
        discriminator: _,
    } = idl_event;

    // We use a similar approach here as we used for accounts (see ./idl_type_definition.rs try_convert_from_account try_convert_from_account)
    IdlEvent {
        name: name.clone(),
        fields: vec![IdlEventField {
            name: "defined".to_string(),
            ty: IdlType::Defined(name),
            // Not sure what index was for, but we don't have that info anyways
            index: false,
        }],
    }
}
