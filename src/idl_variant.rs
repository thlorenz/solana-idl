use serde::{Deserialize, Serialize};

use crate::{idl_field::IdlField, idl_type::IdlType};

/// Underlying fields of a tuple or struct [IdlEnumVariant].
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum EnumFields {
    Named(Vec<IdlField>),
    Tuple(Vec<IdlType>),
}

/// An enum variant which could be scalar (withouth fields) or tuple/struct (with fields).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IdlEnumVariant {
    /// Name of the variant.
    pub name: String,

    /// Optional fields of the variant, only present when it is a tuple or a struct variant.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub fields: Option<EnumFields>,
}
