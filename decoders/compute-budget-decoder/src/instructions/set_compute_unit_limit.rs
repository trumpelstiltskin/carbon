use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x02")]
pub struct SetComputeUnitLimit {
    pub units: u32,
}

pub struct SetComputeUnitLimitInstructionAccounts;

impl carbon_core::deserialize::ArrangeAccounts for SetComputeUnitLimit {
    type ArrangedAccounts = SetComputeUnitLimitInstructionAccounts;

    fn arrange_accounts(
        _accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        Some(SetComputeUnitLimitInstructionAccounts)
    }
}
