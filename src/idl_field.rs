use serde::{Deserialize, Serialize};

use crate::idl_type::IdlType;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IdlField {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: IdlType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<Vec<String>>,
}
