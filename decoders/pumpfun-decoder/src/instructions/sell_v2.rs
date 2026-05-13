use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5df6823ce7e940b2")]
pub struct SellV2 {
    pub amount: u64,
    pub min_sol_output: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SellV2InstructionAccounts {
    pub global: solana_pubkey::Pubkey,
    pub base_mint: solana_pubkey::Pubkey,
    pub quote_mint: solana_pubkey::Pubkey,
    pub base_token_program: solana_pubkey::Pubkey,
    pub quote_token_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub fee_recipient: solana_pubkey::Pubkey,
    pub associated_quote_fee_recipient: solana_pubkey::Pubkey,
    pub buyback_fee_recipient: solana_pubkey::Pubkey,
    pub associated_quote_buyback_fee_recipient: solana_pubkey::Pubkey,
    pub bonding_curve: solana_pubkey::Pubkey,
    pub associated_base_bonding_curve: solana_pubkey::Pubkey,
    pub associated_quote_bonding_curve: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub associated_base_user: solana_pubkey::Pubkey,
    pub associated_quote_user: solana_pubkey::Pubkey,
    pub creator_vault: solana_pubkey::Pubkey,
    pub associated_creator_vault: solana_pubkey::Pubkey,
    pub sharing_config: solana_pubkey::Pubkey,
    pub user_volume_accumulator: solana_pubkey::Pubkey,
    pub associated_user_volume_accumulator: solana_pubkey::Pubkey,
    pub fee_config: solana_pubkey::Pubkey,
    pub fee_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SellV2 {
    type ArrangedAccounts = SellV2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let global = next_account(&mut iter)?;
        let base_mint = next_account(&mut iter)?;
        let quote_mint = next_account(&mut iter)?;
        let base_token_program = next_account(&mut iter)?;
        let quote_token_program = next_account(&mut iter)?;
        let associated_token_program = next_account(&mut iter)?;
        let fee_recipient = next_account(&mut iter)?;
        let associated_quote_fee_recipient = next_account(&mut iter)?;
        let buyback_fee_recipient = next_account(&mut iter)?;
        let associated_quote_buyback_fee_recipient = next_account(&mut iter)?;
        let bonding_curve = next_account(&mut iter)?;
        let associated_base_bonding_curve = next_account(&mut iter)?;
        let associated_quote_bonding_curve = next_account(&mut iter)?;
        let user = next_account(&mut iter)?;
        let associated_base_user = next_account(&mut iter)?;
        let associated_quote_user = next_account(&mut iter)?;
        let creator_vault = next_account(&mut iter)?;
        let associated_creator_vault = next_account(&mut iter)?;
        let sharing_config = next_account(&mut iter)?;
        let user_volume_accumulator = next_account(&mut iter)?;
        let associated_user_volume_accumulator = next_account(&mut iter)?;
        let fee_config = next_account(&mut iter)?;
        let fee_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(SellV2InstructionAccounts {
            global,
            base_mint,
            quote_mint,
            base_token_program,
            quote_token_program,
            associated_token_program,
            fee_recipient,
            associated_quote_fee_recipient,
            buyback_fee_recipient,
            associated_quote_buyback_fee_recipient,
            bonding_curve,
            associated_base_bonding_curve,
            associated_quote_bonding_curve,
            user,
            associated_base_user,
            associated_quote_user,
            creator_vault,
            associated_creator_vault,
            sharing_config,
            user_volume_accumulator,
            associated_user_volume_accumulator,
            fee_config,
            fee_program,
            system_program,
            event_authority,
            program,
        })
    }
}
