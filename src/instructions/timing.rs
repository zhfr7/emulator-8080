use crate::state::State;

use super::{
    ArithmeticInstruction as AI, BranchInstruction as BI, DataTransferInstruction as DTI,
    Instruction, LogicalInstruction as LI, MachineControlInstruction as MCI,
};

pub fn get_instruction_timing(state: &State, instruction: &Instruction) -> usize {
    match instruction {
        Instruction::DataTransfer(dti) => match dti {
            DTI::Move(_, _) => 5,
            DTI::MoveFromMem(_) | DTI::MoveToMem(_) | DTI::MoveImmediate(_, _) => 7,
            DTI::MoveToMemImmediate(_) | DTI::LoadRegisterPairImmediate(_, _) => 10,
            DTI::LoadAccumDirect(_) | DTI::StoreAccumDirect(_) => 13,
            DTI::LoadHLDirect(_) | DTI::StoreHLDirect(_) => 16,
            DTI::LoadAccumIndirect(_) | DTI::StoreAccumIndirect(_) => 7,
            DTI::ExchangeHLWithDE => 4,
        },
        Instruction::Arithmetic(ai) => match ai {
            AI::Add(_) => 4,
            AI::AddMem | AI::AddImmediate(_) => 7,
            AI::AddWithCarry(_) => 4,
            AI::AddMemWithCarry | AI::AddImmediateWithCarry(_) => 7,
            AI::Subtract(_) => 4,
            AI::SubtractMem | AI::SubtractImmediate(_) => 7,
            AI::SubtractWithBorrow(_) => 4,
            AI::SubtractMemWithBorrow | AI::SubtractImmediateWithBorrow(_) => 7,
            AI::Increment(_) => 5,
            AI::IncrementMem => 10,
            AI::Decrement(_) => 5,
            AI::DecrementMem => 10,
            AI::IncrementRegPair(_) | AI::DecrementRegPair(_) => 5,
            AI::AddRegPairToHL(_) => 10,
            AI::DecimalAdjustAccum => 4,
        },
        Instruction::Logical(li) => match li {
            LI::And(_) => 4,
            LI::AndMem | LI::AndImmediate(_) => 7,
            LI::Xor(_) => 4,
            LI::XorMem | LI::XorImmediate(_) => 7,
            LI::Or(_) => 4,
            LI::OrMem | LI::OrImmediate(_) => 7,
            LI::Compare(_) => 4,
            LI::CompareMem | LI::CompareImmediate(_) => 7,
            _ => 4,
        },
        Instruction::Branch(bi) => match bi {
            BI::Jump(_) | BI::ConditionalJump(_, _) => 10,
            BI::Call(_) => 17,
            BI::ConditionalCall(condition, _) => {
                if state.registers.is_condition_fulfilled(condition) {
                    17
                } else {
                    11
                }
            }
            BI::Return => 10,
            BI::ConditionalReturn(condition) => {
                if state.registers.is_condition_fulfilled(condition) {
                    11
                } else {
                    5
                }
            }
            BI::Restart(_) => 11,
            BI::JumpHLIndirect => 5,
        },
        Instruction::MachineControl(mci) => match mci {
            MCI::PushRegPair(_) | MCI::PushPSW => 11,
            MCI::PopRegPair(_) | MCI::PopPSW => 10,
            MCI::ExchangeStackTopWithHL => 18,
            MCI::MoveHLToSP => 5,
            MCI::Input(_) | MCI::Output(_) => 10,
            MCI::EnableInterrupts | MCI::DisableInterrupts => 4,
            MCI::Halt => 7,
            MCI::NoOp => 4,
        },
    }
}
