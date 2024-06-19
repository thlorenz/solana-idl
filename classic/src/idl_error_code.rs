use serde::{Deserialize, Serialize};

/// Error code and info for a specific error emitted by the program.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IdlErrorCode {
    /// Code of the error that is returned as part of the transaction.
    pub code: u32,
    /// The user friendly name of the error.
    pub name: String,
    /// An optional error description.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub msg: Option<String>,
}
