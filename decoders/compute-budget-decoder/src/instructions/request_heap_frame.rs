use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x01")]
pub struct RequestHeapFrame {
    pub bytes: u32,
}

pub struct RequestHeapFrameInstructionAccounts;

impl carbon_core::deserialize::ArrangeAccounts for RequestHeapFrame {
    type ArrangedAccounts = RequestHeapFrameInstructionAccounts;

    fn arrange_accounts(
        _accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        Some(RequestHeapFrameInstructionAccounts)
    }
}
