use crate::{idl_field::IdlField, idl_variant::IdlEnumVariant};
use serde::{Deserialize, Serialize};

// -----------------
// IdlTypeDefinitionTy
// -----------------

/// The underlying type of a [IdlTypeDefinition], namely an enum with variants or a struct with
/// fields.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase", tag = "kind")]
pub enum IdlTypeDefinitionTy {
    Struct { fields: Vec<IdlField> },
    Enum { variants: Vec<IdlEnumVariant> },
}

// -----------------
// IdlTypeDefinition
// -----------------

/// Custom type definition.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IdlTypeDefinition {
    /// Name of the struct or enum.
    pub name: String,

    /// Underlying type description.
    #[serde(rename = "type")]
    pub ty: IdlTypeDefinitionTy,
}
