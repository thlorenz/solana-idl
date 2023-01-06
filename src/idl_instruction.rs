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
    pub name: String,
    pub accounts: Vec<IdlAccountItem>,
    pub args: Vec<IdlField>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub default_optional_accounts: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub discriminant: Option<IdlInstructionDiscriminant>,
}

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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IdlAccounts {
    pub name: String,
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IdlAccount {
    pub name: String,
    pub is_mut: bool,
    pub is_signer: bool,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub desc: Option<String>,
    #[serde(skip_serializing_if = "is_false", default)]
    pub optional: bool,
}
