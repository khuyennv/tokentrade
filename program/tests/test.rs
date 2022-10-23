use solana_program::system_program;
use solana_program_test::*;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::Signer,
    transaction::Transaction,
    account::Account,
};
use std::mem;
use std::str::FromStr;
use spl_associated_token_account::{
    get_associated_token_address, instruction::create_associated_token_account,
};
use spl_token;
use tokentrade::entrypoint::process_instruction;

#[tokio::test]
async fn test_initialize_false() {
    let program_id = Pubkey::new_unique();
    let mint = Pubkey::new_unique();

    let (vault, _bump_seed) =
        Pubkey::find_program_address(&[b"vault", &mint.to_bytes()], &program_id);

    let mut program_test = ProgramTest::new(
        "tokentrade", // Run the BPF version with `cargo test-bpf`
        program_id,
        processor!(process_instruction), // Run the native version with `cargo test`
    );

    let (mut banks_client, payer, block_hash) = program_test.start().await;

    // vault does not exist
    assert_eq!(
        banks_client.get_account(vault).await.unwrap().is_none(),
        true,
    );
}

#[tokio::test]
async fn test_initialize_success() {
    let program_id = Pubkey::new_unique();
    let greeted_pubkey = Pubkey::new_unique();

    let mut program_test = ProgramTest::new(
        "tokentrade", // Run the BPF version with `cargo test-bpf`
        program_id,
        processor!(process_instruction), // Run the native version with `cargo test`
    );

    program_test.add_account(
        greeted_pubkey,
        Account {
            lamports: 5,
            data: vec![0_u8; mem::size_of::<u32>()],
            owner: program_id,
            ..Account::default()
        },
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    let account = banks_client
        .get_account(greeted_pubkey)
        .await;

    assert_eq!(
        account.unwrap().is_some(),
        true
    );
}

#[tokio::test]
async fn test_transfer_sol_to_token() {
    let program_id = Pubkey::new_unique();
    let mint = Pubkey::new_unique();

    let (vault, _bump_seed) =
        Pubkey::find_program_address(&[b"vault", &mint.to_bytes()], &program_id);

    let vault_ata = get_associated_token_address(&vault, &mint);
    let mut program_test = ProgramTest::new(
        "tokentrade", // Run the BPF version with `cargo test-bpf`
        program_id,
        processor!(process_instruction), // Run the native version with `cargo test`
    );

    program_test.add_account(
        vault,
        Account {
            lamports: 1,
            data: vec![0],
            owner: program_id,
            ..Account::default()
        },
    );

    let (mut banks_client, payer, block_hash) = program_test.start().await;

    let payer_ata = get_associated_token_address(&payer.pubkey(), &mint);

    let vault_account =  banks_client.get_account(vault).await.unwrap();
    // println!("->>>>>>>>>>>>>>>>>>>>{:#?}", vault_account);

    assert_eq!(
        vault_account.is_some(),
        true
    );

    let mut init_tx = Transaction::new_with_payer(
        &[Instruction::new_with_bincode(
            program_id,
            &[0],
            vec![
                AccountMeta::new(payer.pubkey(), false),
                AccountMeta::new(vault, false),
                AccountMeta::new(mint, false),
                AccountMeta::new(system_program::id(), false),
            ],
        )],
        Some(&payer.pubkey()),
    );
    init_tx.sign(&[&payer], block_hash);

    let result_tran = banks_client.process_transaction(init_tx).await;

    // println!("{:#?}", result_tran);

    let acc = banks_client.get_account(vault).await.unwrap();

    // vault created successfully on the system
    assert_eq!(
        acc.is_some(),
        true,
    );
    let sol_in_lamparts_amount = 100000000000;
    let arr = u64::to_le_bytes(sol_in_lamparts_amount);
    let mut instruction_data = [1; 9];
    for i in 0..8 {
        instruction_data[i + 1] = arr[i];
    }

    let mut swap_tx = Transaction::new_with_payer(
        &[Instruction::new_with_bincode(
            program_id,
            &instruction_data,
            vec![
                AccountMeta::new(payer.pubkey(), true),
                AccountMeta::new(payer_ata, false),
                AccountMeta::new(program_id, false),
                AccountMeta::new(mint, false),
                AccountMeta::new(vault, false),
                AccountMeta::new(vault_ata, false),
                AccountMeta::new_readonly(spl_token::id(), false),
                AccountMeta::new(system_program::id(), false),
            ],
        )],
        Some(&payer.pubkey()),
    );

    swap_tx.sign(&[&payer], block_hash);

    let vault_balance_before = banks_client.get_balance(vault).await;

    banks_client.process_transaction(swap_tx).await.unwrap();

    let vault_balance_after = banks_client.get_balance(vault).await;

    println!("{:#?} months in a year.", vault_balance_after);

    assert_eq!(
        vault_balance_before.unwrap() + sol_in_lamparts_amount,
        vault_balance_after.unwrap()
    );
}
