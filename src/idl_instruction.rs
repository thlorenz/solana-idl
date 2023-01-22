use crate::{idl_field::IdlField, idl_type::IdlType};
use serde::{Deserialize, Serialize};

// -----------------
// IdlInstructions
// -----------------
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IdlInstructions(pub Vec<IdlInstruction>);

// -----------------
// IdlInstruction
// -----------------

/// This represents one Instruction which in the case of ShankInstruction is just
/// one variant of that enum.
/// We also expect it to only have one arg which is a custom type containing the
/// respective instruction args.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IdlInstruction {
    /// Name of the instruction.
    pub name: String,

    /// Accounts that need to be supplied in order to process the instruction.
    pub accounts: Vec<IdlAccountItem>,

    /// Instruction args.
    pub args: Vec<IdlField>,

    /// Shank only.
    ///
    /// When the `#[default_optional_accounts]` attribute is added to an Instruction enum, shank will mark it
    /// such that optional accounts should default to the `progam_id` if they are not provided by the client.
    /// Thus their position is static and optional accounts that are set can follow ones that are not.
    ///
    /// The default strategy (without `#[default_optional_accounts]`) is to just omit unset optional
    /// accounts from the accounts array.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub default_optional_accounts: Option<bool>,

    /// A custom discriminant for the instruction, defaults to the index of the variant in the enum.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub discriminant: Option<IdlInstructionDiscriminant>,
}

/// A discriminant for an instruction.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IdlInstructionDiscriminant {
    #[serde(rename = "type")]
    pub ty: IdlType,
    pub value: u8,
}

impl From<u8> for IdlInstructionDiscriminant {
    fn from(value: u8) -> Self {
        Self {
            ty: IdlType::U8,
            value,
        }
    }
}

// -----------------
// IdlAccounts
// -----------------

/// Accounts provided when calling an instruction.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IdlAccounts {
    pub name: String,
    /// Accounts to provide.
    pub accounts: Vec<IdlAccountItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum IdlAccountItem {
    IdlAccount(IdlAccount),
    IdlAccounts(IdlAccounts),
}

fn is_false(x: &bool) -> bool {
    return !x;
}

// -----------------
// IdlAccount
// -----------------
/// Metadata of an account that is provided when calling an instruction.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IdlAccount {
    /// Name of the account used for documentation and by code generators.
    pub name: String,

    /// Whether the account is writable.
    pub is_mut: bool,

    /// Whether the account is signer.
    pub is_signer: bool,

    /// Description of the account used for documentation and by code generators.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub desc: Option<String>,

    /// Whether the account is optional or not.
    #[serde(skip_serializing_if = "is_false", default)]
    pub optional: bool,
}
