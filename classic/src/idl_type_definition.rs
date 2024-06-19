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

impl IdlTypeDefinitionTy {
    pub fn is_struct(&self) -> bool {
        matches!(self, IdlTypeDefinitionTy::Struct { .. })
    }

    pub fn is_enum(&self) -> bool {
        matches!(self, IdlTypeDefinitionTy::Enum { .. })
    }

    pub fn fields(&self) -> Option<&Vec<IdlField>> {
        match self {
            IdlTypeDefinitionTy::Struct { fields } => Some(fields),
            _ => None,
        }
    }

    pub fn variants(&self) -> Option<&Vec<IdlEnumVariant>> {
        match self {
            IdlTypeDefinitionTy::Enum { variants } => Some(variants),
            _ => None,
        }
    }
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
