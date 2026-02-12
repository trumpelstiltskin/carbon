use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x00")]
pub struct RequestUnits {
    pub units: u32,
    pub additional_fee: u32,
}

pub struct RequestUnitsInstructionAccounts;

impl carbon_core::deserialize::ArrangeAccounts for RequestUnits {
    type ArrangedAccounts = RequestUnitsInstructionAccounts;

    fn arrange_accounts(
        _accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        Some(RequestUnitsInstructionAccounts)
    }
}
