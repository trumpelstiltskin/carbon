use super::ComputeBudgetDecoder;
pub mod request_heap_frame;
pub mod request_units;
pub mod set_compute_unit_limit;
pub mod set_compute_unit_price;

#[derive(
    carbon_core::InstructionType,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Debug,
    Clone,
    Eq,
    Hash,
)]
pub enum ComputeBudgetInstruction {
    RequestUnits(request_units::RequestUnits),
    RequestHeapFrame(request_heap_frame::RequestHeapFrame),
    SetComputeUnitLimit(set_compute_unit_limit::SetComputeUnitLimit),
    SetComputeUnitPrice(set_compute_unit_price::SetComputeUnitPrice),
}

impl carbon_core::instruction::InstructionDecoder<'_> for ComputeBudgetDecoder {
    type InstructionType = ComputeBudgetInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&super::PROGRAM_ID) {
            return None;
        }

        carbon_core::try_decode_instructions!(instruction,
            ComputeBudgetInstruction::RequestUnits => request_units::RequestUnits,
            ComputeBudgetInstruction::RequestHeapFrame => request_heap_frame::RequestHeapFrame,
            ComputeBudgetInstruction::SetComputeUnitLimit => set_compute_unit_limit::SetComputeUnitLimit,
            ComputeBudgetInstruction::SetComputeUnitPrice => set_compute_unit_price::SetComputeUnitPrice,
        )
    }
}
