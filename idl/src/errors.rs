use thiserror::Error;

pub type IdlResult<T> = std::result::Result<T, IdlError>;

#[derive(Debug, Error)]
pub enum IdlError {
    #[error("IdlConverterError: {0}")]
    IdlConverterError(#[from] solana_idl_converter::errors::IdlConverterError),

    #[error("Idl could not be parsed as classic ('{0}') or new ('{1}')")]
    IdlCouldNotBeParsed(serde_json::Error, serde_json::Error),

    #[error("Idl could not be parsed as classic ('{0}') or converted from new ('{1}')")]
    IdlCouldNotBeConverted(
        serde_json::Error,
        solana_idl_converter::errors::IdlConverterError,
    ),
}
