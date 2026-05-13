use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct Shareholder {
    pub address: solana_pubkey::Pubkey,
    pub share_bps: u16,
}
