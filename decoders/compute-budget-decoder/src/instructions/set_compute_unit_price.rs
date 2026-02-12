use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x03")]
pub struct SetComputeUnitPrice {
    pub micro_lamports: u64,
}

pub struct SetComputeUnitPriceInstructionAccounts;

impl carbon_core::deserialize::ArrangeAccounts for SetComputeUnitPrice {
    type ArrangedAccounts = SetComputeUnitPriceInstructionAccounts;

    fn arrange_accounts(
        _accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        Some(SetComputeUnitPriceInstructionAccounts)
    }
}
