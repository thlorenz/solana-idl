use thiserror::Error;

pub type IdlConverterResult<T> = std::result::Result<T, IdlConverterError>;

#[derive(Debug, Error)]
pub enum IdlConverterError {
    #[error("The IDL type: '{0}' is not supported in the classic IDL and cannot be converted. Context: '{1}'")]
    UnsupportedClassicIdlType(String, String),
}
