use anchor_lang_idl::types::{
    IdlDiscriminator as NewIdlDiscriminator,
    IdlInstruction as NewIdlInstruction,
    IdlInstructionAccount as NewIdlInstructionAccount,
    IdlInstructionAccountItem as NewIdlInstructionAccountItem,
    IdlInstructionAccounts as NewIdlInstructionAccounts,
};
use solana_idl_classic::{
    IdlAccount, IdlAccountItem, IdlAccounts, IdlInstruction,
    IdlInstructionDiscriminant, IdlType,
};

use super::idl_field;
use crate::errors::IdlConverterResult;

pub fn try_convert(
    idl_instruction: NewIdlInstruction,
) -> IdlConverterResult<IdlInstruction> {
    let NewIdlInstruction {
        name,
        docs: _,
        discriminator,
        accounts,
        args,
        returns: _,
    } = idl_instruction;
    let accounts = accounts
        .into_iter()
        .map(convert_instruction_account_item)
        .collect();
    let args = args
        .into_iter()
        .enumerate()
        .map(|(idx, arg)| {
            let context = format!("Instruction '{}' arg[{}]", name, idx);
            idl_field::try_convert(arg, &context)
        })
        .collect::<IdlConverterResult<Vec<_>>>()?;
    let discriminant = convert_discriminant(discriminator);
    Ok(IdlInstruction {
        name,
        accounts,
        args,
        default_optional_accounts: None,
        discriminant,
    })
}

// -----------------
// Discriminator
// -----------------
fn convert_discriminant(
    idl_discriminator: NewIdlDiscriminator,
) -> Option<IdlInstructionDiscriminant> {
    // NOTE: it's impossible to properly convert a Vec<u8> to a u8, so we do our
    // best attempt, however we include the full bytes as well so that tools
    // that deserialize instruction IDLs have access to it.
    idl_discriminator
        .first()
        .cloned()
        .map(|first| IdlInstructionDiscriminant {
            ty: IdlType::U8,
            value: first,
            bytes: Some(idl_discriminator),
        })
}

// -----------------
// InstructionAccounts
// -----------------
fn convert_instruction_account_item(
    idl_instruction_account_item: NewIdlInstructionAccountItem,
) -> IdlAccountItem {
    use NewIdlInstructionAccountItem::*;
    match idl_instruction_account_item {
        Single(account) => {
            IdlAccountItem::IdlAccount(convert_instruction_account(account))
        }
        Composite(accounts) => {
            IdlAccountItem::IdlAccounts(convert_instruction_accounts(accounts))
        }
    }
}

fn convert_instruction_accounts(
    accounts: NewIdlInstructionAccounts,
) -> IdlAccounts {
    IdlAccounts {
        name: accounts.name,
        accounts: accounts
            .accounts
            .into_iter()
            .map(convert_instruction_account_item)
            .collect(),
    }
}
fn convert_instruction_account(
    idl_instruction_account: NewIdlInstructionAccount,
) -> IdlAccount {
    let NewIdlInstructionAccount {
        name,
        docs,
        writable,
        signer,
        optional,
        address,
        pda: _,
        relations: _,
    } = idl_instruction_account;

    IdlAccount {
        name,
        is_mut: writable,
        is_signer: signer,
        optional,
        desc: None,
        docs: Some(docs),
        address,
    }
}
