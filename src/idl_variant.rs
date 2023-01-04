use serde::{Deserialize, Serialize};

use crate::{idl_field::IdlField, idl_type::IdlType};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum EnumFields {
    Named(Vec<IdlField>),
    Tuple(Vec<IdlType>),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IdlEnumVariant {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub fields: Option<EnumFields>,
}
