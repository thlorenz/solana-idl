use anchor_lang_idl::types::IdlErrorCode as NewIdlErrorCode;
use solana_idl_classic::IdlErrorCode;

pub fn convert(error_code: NewIdlErrorCode) -> IdlErrorCode {
    IdlErrorCode {
        code: error_code.code,
        name: error_code.name,
        msg: error_code.msg,
    }
}
