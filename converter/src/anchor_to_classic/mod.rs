use anchor_lang_idl::types::Idl as NewIdl;
use solana_idl_classic::Idl;
mod idl_defined_fields;
mod idl_field;
mod idl_type;
mod idl_type_definition;
mod idl_variant;

pub fn convert(idl: NewIdl) -> Idl {
    todo!()
}
