use serde::{Deserialize, Serialize};

/// IDL Metadata only available with Shank.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IdlMetadata {
    /// Origin of this IDL, i.e. if anchor or shank extracted it.
    pub origin: String,

    /// The address of the program this IDL describes.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub address: Option<String>,

    /// The serializer used to encode and decode accounts, i.e. borsh.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub serializer: Option<String>,
}
