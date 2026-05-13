use std::sync::Arc;

use carbon_core::{
    datasource::TransactionUpdate,
    deserialize::ArrangeAccounts,
    instruction::InstructionDecoder,
    transaction::TransactionMetadata,
    transformers::{extract_instructions_with_metadata, transaction_metadata_from_original_meta},
};
use carbon_pumpfun_decoder::{
    instructions::{buy_exact_quote_in_v2, buy_v2, migrate, sell_v2, PumpfunInstruction},
    PumpfunDecoder,
};
use solana_instruction::AccountMeta;
use solana_pubkey::Pubkey;
use solana_transaction_status::EncodedConfirmedTransactionWithStatusMeta;

fn decoded_pumpfun_instructions(json: &str) -> Vec<PumpfunInstruction> {
    let value: serde_json::Value = serde_json::from_str(json).expect("parse fixture JSON");
    let confirmed: EncodedConfirmedTransactionWithStatusMeta =
        serde_json::from_value(value["result"].clone()).expect("deserialize transaction");
    let encoded = confirmed.transaction;
    let transaction = encoded.transaction.decode().expect("decode transaction");
    let meta = transaction_metadata_from_original_meta(encoded.meta.expect("transaction meta"))
        .expect("convert transaction meta");
    let update = TransactionUpdate {
        signature: *transaction
            .signatures
            .first()
            .expect("transaction signature"),
        transaction,
        meta,
        is_vote: false,
        slot: confirmed.slot,
        index: Some(0),
        block_time: confirmed.block_time,
        block_hash: None,
    };
    let metadata = Arc::new(TransactionMetadata::try_from(update.clone()).expect("metadata"));
    let decoder = PumpfunDecoder;

    extract_instructions_with_metadata(&metadata, &update)
        .expect("extract instructions")
        .into_iter()
        .filter_map(|(_, instruction)| decoder.decode_instruction(&instruction))
        .map(|decoded| decoded.data)
        .collect()
}

fn instruction_names(instructions: &[PumpfunInstruction]) -> Vec<&'static str> {
    instructions
        .iter()
        .map(|instruction| match instruction {
            PumpfunInstruction::BuyV2(_) => "BuyV2",
            PumpfunInstruction::SellV2(_) => "SellV2",
            PumpfunInstruction::BuyExactQuoteInV2(_) => "BuyExactQuoteInV2",
            PumpfunInstruction::CreateV2(_) => "CreateV2",
            PumpfunInstruction::CreateEvent(_) => "CreateEvent",
            PumpfunInstruction::TradeEvent(_) => "TradeEvent",
            _ => "Other",
        })
        .collect()
}

#[test]
fn decodes_buy_v2_transaction_json_and_trade_event() {
    let instructions =
        decoded_pumpfun_instructions(include_str!("fixtures/mainnet_buy_v2_tx.json"));

    assert_eq!(instruction_names(&instructions), ["BuyV2", "TradeEvent"]);
    assert!(matches!(
        &instructions[0],
        PumpfunInstruction::BuyV2(buy_v2::BuyV2 {
            amount: 13_296_580_381,
            ..
        })
    ));

    let PumpfunInstruction::TradeEvent(trade) = &instructions[1] else {
        panic!("expected trade event");
    };
    assert!(trade.is_buy);
    assert_eq!(trade.ix_name, "buy");
    assert_eq!(trade.sol_amount, 2_829_634);
    assert_eq!(trade.token_amount, 13_296_580_381);
    assert_eq!(trade.buyback_fee_basis_points, 5_000);
    assert_eq!(trade.shareholders.len(), 1);
    assert_eq!(trade.quote_mint, Pubkey::default());
    assert_eq!(trade.quote_amount, 2_829_634);
    assert_eq!(trade.virtual_quote_reserves, 82_768_101_455);
    assert_eq!(trade.real_quote_reserves, 52_768_101_455);
}

#[test]
fn decodes_sell_v2_transaction_json_and_trade_event() {
    let instructions =
        decoded_pumpfun_instructions(include_str!("fixtures/mainnet_sell_v2_tx.json"));

    assert_eq!(instruction_names(&instructions), ["SellV2", "TradeEvent"]);
    assert!(matches!(
        &instructions[0],
        PumpfunInstruction::SellV2(sell_v2::SellV2 {
            amount: 6_100_266_141_751,
            ..
        })
    ));

    let PumpfunInstruction::TradeEvent(trade) = &instructions[1] else {
        panic!("expected trade event");
    };
    assert!(!trade.is_buy);
    assert_eq!(trade.ix_name, "sell");
    assert_eq!(trade.sol_amount, 877_687_475);
    assert_eq!(trade.token_amount, 6_100_266_141_751);
    assert_eq!(trade.cashback_fee_basis_points, 30);
    assert_eq!(trade.buyback_fee_basis_points, 5_000);
    assert!(trade.shareholders.is_empty());
    assert_eq!(trade.quote_mint, Pubkey::default());
    assert_eq!(trade.quote_amount, 877_687_475);
    assert_eq!(trade.virtual_quote_reserves, 67_616_946_207);
    assert_eq!(trade.real_quote_reserves, 37_616_946_207);
}

#[test]
fn decodes_create_and_buy_exact_quote_in_v2_transaction_json_events() {
    let instructions = decoded_pumpfun_instructions(include_str!(
        "fixtures/mainnet_create_buy_exact_quote_in_v2_tx.json"
    ));

    assert_eq!(
        instruction_names(&instructions),
        ["CreateV2", "CreateEvent", "BuyExactQuoteInV2", "TradeEvent"]
    );
    assert!(matches!(
        &instructions[2],
        PumpfunInstruction::BuyExactQuoteInV2(buy_exact_quote_in_v2::BuyExactQuoteInV2 {
            spendable_quote_in: 30_000_000,
            min_tokens_out: 1,
        })
    ));

    let PumpfunInstruction::CreateEvent(create) = &instructions[1] else {
        panic!("expected create event");
    };
    assert_eq!(create.name, "Charlie Kirk Coin");
    assert_eq!(create.symbol, "KIRK");
    assert_eq!(create.quote_mint, Pubkey::default());
    assert_eq!(create.virtual_quote_reserves, 30_000_000_000);

    let PumpfunInstruction::TradeEvent(trade) = &instructions[3] else {
        panic!("expected trade event");
    };
    assert!(trade.is_buy);
    assert_eq!(trade.ix_name, "buy_exact_quote_in");
    assert_eq!(trade.sol_amount, 29_629_629);
    assert_eq!(trade.token_amount, 1_058_707_391_261);
    assert_eq!(trade.buyback_fee_basis_points, 5_000);
    assert_eq!(trade.quote_mint, Pubkey::default());
    assert_eq!(trade.quote_amount, 29_629_629);
    assert_eq!(trade.virtual_quote_reserves, 30_029_629_629);
    assert_eq!(trade.real_quote_reserves, 29_629_629);
}

#[test]
fn migrate_arranges_rent_account_from_current_idl() {
    let accounts = (0..25)
        .map(|index| AccountMeta::new(Pubkey::new_from_array([index; 32]), false))
        .collect::<Vec<_>>();

    let arranged = migrate::Migrate::arrange_accounts(&accounts).expect("arrange accounts");

    assert_eq!(arranged.program, accounts[23].pubkey);
    assert_eq!(arranged.rent, accounts[24].pubkey);
}
