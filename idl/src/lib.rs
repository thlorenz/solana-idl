use anchor_lang_idl::types::Idl as NewIdl;
use errors::{IdlError, IdlResult};
use solana_idl_classic::Idl;
use solana_idl_converter::anchor_to_classic;

pub mod errors;
pub fn try_extract_classic_idl(json: &str) -> IdlResult<Idl> {
    // First try to parse the JSON as a classic IDL
    match serde_json::from_str::<Idl>(json) {
        Ok(idl) => Ok(idl),
        Err(parse_as_classic_err) => {
            // If that fails, try to parse it as a new IDL
            let new_idl = match serde_json::from_str::<NewIdl>(json) {
                Ok(new_idl) => new_idl,
                Err(parse_as_new_err) => {
                    return Err(IdlError::IdlCouldNotBeParsed(
                        parse_as_classic_err,
                        parse_as_new_err,
                    ))
                }
            };
            anchor_to_classic::try_convert(new_idl).map_err(|conversion_err| {
                IdlError::IdlCouldNotBeConverted(
                    parse_as_classic_err,
                    conversion_err,
                )
            })
        }
    }
}
