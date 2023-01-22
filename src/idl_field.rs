use serde::{Deserialize, Serialize};

use crate::idl_type::IdlType;

/// A field in a struct, enum variant or [IdlInstruction] args.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IdlField {
    /// Name of the field.
    pub name: String,

    /// Type of the field.
    #[serde(rename = "type")]
    pub ty: IdlType,

    /// Attributes with which field was annotated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<Vec<String>>,
}
