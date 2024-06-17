use anchor_lang_idl::types::{IdlArrayLen, IdlType as NewIdlType};
use solana_idl_classic::IdlType;

use crate::errors::{IdlConverterError, IdlConverterResult};

pub fn try_convert(idl_type: NewIdlType) -> IdlConverterResult<IdlType> {
    use NewIdlType::*;
    match idl_type {
        Bool => Ok(IdlType::Bool),
        U8 => Ok(IdlType::U8),
        I8 => Ok(IdlType::I8),
        U16 => Ok(IdlType::U16),
        I16 => Ok(IdlType::I16),
        U32 => Ok(IdlType::U32),
        I32 => Ok(IdlType::I32),
        F32 => Ok(IdlType::F32),
        U64 => Ok(IdlType::U64),
        I64 => Ok(IdlType::I64),
        F64 => Ok(IdlType::F64),
        U128 => Ok(IdlType::U128),
        I128 => Ok(IdlType::I128),
        U256 => Err(IdlConverterError::UnsupportedClassicIdlType(
            "U256".to_string(),
        )),
        I256 => Err(IdlConverterError::UnsupportedClassicIdlType(
            "I256".to_string(),
        )),
        Bytes => Ok(IdlType::Bytes),
        String => Ok(IdlType::String),
        Pubkey => Ok(IdlType::PublicKey),
        Option(inner) => {
            let inner = try_convert(*inner)?;
            Ok(IdlType::Option(Box::new(inner)))
        }
        Vec(inner) => {
            let inner = try_convert(*inner)?;
            Ok(IdlType::Vec(Box::new(inner)))
        }
        Array(inner, array_len) => {
            let size = match array_len {
                IdlArrayLen::Generic(_) => {
                    return Err(IdlConverterError::UnsupportedClassicIdlType(
                        "IdlArrayLen::Generic".to_string(),
                    ))
                }
                IdlArrayLen::Value(val) => val,
            };
            let inner = try_convert(*inner)?;
            Ok(IdlType::Array(Box::new(inner), size))
        }
        Defined { name, generics: _ } => Ok(IdlType::Defined(name)),
        Generic(name) => Err(IdlConverterError::UnsupportedClassicIdlType(format!(
            "Generic({})",
            name
        ))),
        added_type => Err(IdlConverterError::UnsupportedClassicIdlType(format!(
            "Unknown type {:?}",
            added_type
        ))),
    }
}
