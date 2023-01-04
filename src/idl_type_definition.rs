use crate::{idl_field::IdlField, idl_variant::IdlEnumVariant};
use serde::{Deserialize, Serialize};

// -----------------
// IdlTypeDefinitionTy
// -----------------
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase", tag = "kind")]
pub enum IdlTypeDefinitionTy {
    Struct { fields: Vec<IdlField> },
    Enum { variants: Vec<IdlEnumVariant> },
}

// -----------------
// IdlTypeDefinition
// -----------------
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IdlTypeDefinition {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: IdlTypeDefinitionTy,
}
