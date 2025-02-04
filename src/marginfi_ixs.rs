use anchor_lang::{InstructionData, Key, ToAccountMetas};

use anchor_spl::token_2022;
use log::trace;
use solana_sdk::instruction::AccountMeta;
use solana_sdk::{instruction::Instruction, pubkey::Pubkey};

pub fn make_deposit_ix(
    marginfi_program_id: Pubkey,
    marginfi_group: Pubkey,
    marginfi_account: Pubkey,
    signer: Pubkey,
    bank: Pubkey,
    signer_token_account: Pubkey,
    bank_liquidity_vault: Pubkey,
    token_program: Pubkey,
    mint: Pubkey,
    amount: u64,
) -> Instruction {
    let mut accounts = marginfi::accounts::LendingAccountDeposit {
        marginfi_group,
        marginfi_account,
        signer,
        bank,
        signer_token_account,
        bank_liquidity_vault,
        token_program,
    }
    .to_account_metas(Some(true));

    maybe_add_bank_mint(&mut accounts, mint, &token_program);

    Instruction {
        program_id: marginfi_program_id,
        accounts,
        data: marginfi::instruction::LendingAccountDeposit { amount }.data(),
    }
}

pub fn make_repay_ix(
    marginfi_program_id: Pubkey,
    marginfi_group: Pubkey,
    marginfi_account: Pubkey,
    signer: Pubkey,
    bank: Pubkey,
    signer_token_account: Pubkey,
    bank_liquidity_vault: Pubkey,
    token_program: Pubkey,
    mint: Pubkey,
    amount: u64,
    repay_all: Option<bool>,
) -> Instruction {
    let mut accounts = marginfi::accounts::LendingAccountRepay {
        marginfi_group,
        marginfi_account,
        signer,
        bank,
        signer_token_account,
        bank_liquidity_vault,
        token_program,
    }
    .to_account_metas(Some(true));

    maybe_add_bank_mint(&mut accounts, mint, &token_program);

    Instruction {
        program_id: marginfi_program_id,
        accounts,
        data: marginfi::instruction::LendingAccountRepay { amount, repay_all }.data(),
    }
}

pub fn make_withdraw_ix(
    marginfi_program_id: Pubkey,
    marginfi_group: Pubkey,
    marginfi_account: Pubkey,
    signer: Pubkey,
    bank: Pubkey,
    destination_token_account: Pubkey,
    bank_liquidity_vault_authority: Pubkey,
    bank_liquidity_vault: Pubkey,
    token_program: Pubkey,
    observation_accounts: Vec<Pubkey>,
    mint: Pubkey,
    amount: u64,
    withdraw_all: Option<bool>,
) -> Instruction {
    let mut accounts = marginfi::accounts::LendingAccountWithdraw {
        marginfi_group,
        marginfi_account,
        signer,
        bank,
        destination_token_account,
        bank_liquidity_vault_authority,
        bank_liquidity_vault,
        token_program,
    }
    .to_account_metas(Some(true));

    maybe_add_bank_mint(&mut accounts, mint, &token_program);

    trace!(
        "make_withdraw_ix: observation_accounts: {:?}",
        observation_accounts
    );

    accounts.extend(
        observation_accounts
            .iter()
            .map(|a| AccountMeta::new_readonly(a.key(), false)),
    );

    Instruction {
        program_id: marginfi_program_id,
        accounts,
        data: marginfi::instruction::LendingAccountWithdraw {
            amount,
            withdraw_all,
        }
        .data(),
    }
}

pub fn make_liquidate_ix(
    marginfi_program_id: Pubkey,
    marginfi_group: Pubkey,
    marginfi_account: Pubkey,
    asset_bank: Pubkey,
    liab_bank: Pubkey,
    signer: Pubkey,
    liquidatee_marginfi_account: Pubkey,
    bank_liquidity_vault_authority: Pubkey,
    bank_liquidity_vault: Pubkey,
    bank_insurance_vault: Pubkey,
    token_program: Pubkey,
    liquidator_observation_accounts: Vec<Pubkey>,
    liquidatee_observation_accounts: Vec<Pubkey>,
    asset_bank_oracle: Pubkey,
    liab_bank_oracle: Pubkey,
    liab_mint: Pubkey,
    asset_amount: u64,
) -> Instruction {
    let mut accounts = marginfi::accounts::LendingAccountLiquidate {
        group: marginfi_group,
        liquidator_marginfi_account: marginfi_account,
        signer,
        liquidatee_marginfi_account,
        bank_liquidity_vault_authority,
        bank_liquidity_vault,
        bank_insurance_vault,
        token_program,
        asset_bank,
        liab_bank,
    }
    .to_account_metas(Some(true));

    maybe_add_bank_mint(&mut accounts, liab_mint, &token_program);

    accounts.extend([
        AccountMeta::new_readonly(asset_bank_oracle, false),
        AccountMeta::new_readonly(liab_bank_oracle, false),
    ]);

    accounts.extend(
        liquidator_observation_accounts
            .iter()
            .map(|a| AccountMeta::new_readonly(a.key(), false)),
    );

    accounts.extend(
        liquidatee_observation_accounts
            .iter()
            .map(|a| AccountMeta::new_readonly(a.key(), false)),
    );

    Instruction {
        program_id: marginfi_program_id,
        accounts,
        data: marginfi::instruction::LendingAccountLiquidate { asset_amount }.data(),
    }
}

fn maybe_add_bank_mint(accounts: &mut Vec<AccountMeta>, mint: Pubkey, token_program: &Pubkey) {
    if token_program == &token_2022::ID {
        accounts.push(AccountMeta::new_readonly(mint, false));
    }
}
