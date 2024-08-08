use crate::internal::state::State;

use super::{Instruction as I, Register};

pub fn get_instruction_timing(state: &State, instruction: &I) -> usize {
    match &instruction {
        I::Move(Register::Memory, _) | I::Move(_, Register::Memory) => 7,
        I::Move(_, _) => 5,
        I::MoveImmediate(Register::Memory, _) => 10,
        I::MoveImmediate(_, _) => 7,
        I::LoadRegisterPairImmediate(_, _) => 10,
        I::LoadAccumDirect(_) | I::StoreAccumDirect(_) => 13,
        I::LoadHLDirect(_) | I::StoreHLDirect(_) => 16,
        I::LoadAccumIndirect(_) | I::StoreAccumIndirect(_) => 7,
        I::ExchangeHLWithDE => 4,
        I::Add(Register::Memory) | I::AddWithCarry(Register::Memory) => 7,
        I::Add(_) | I::AddWithCarry(_) => 4,
        I::AddImmediate(_) | I::AddImmediateWithCarry(_) => 7,
        I::Subtract(Register::Memory) | I::SubtractWithBorrow(Register::Memory) => 7,
        I::Subtract(_) | I::SubtractWithBorrow(_) => 4,
        I::SubtractImmediate(_) | I::SubtractImmediateWithBorrow(_) => 7,
        I::Increment(Register::Memory) => 10,
        I::Increment(_) => 5,
        I::Decrement(Register::Memory) => 10,
        I::Decrement(_) => 5,
        I::IncrementRegPair(_) | I::DecrementRegPair(_) => 5,
        I::AddRegPairToHL(_) => 10,
        I::DecimalAdjustAccum => 4,
        I::And(Register::Memory) | I::AndImmediate(_) => 7,
        I::And(_) => 4,
        I::Xor(Register::Memory) | I::XorImmediate(_) => 7,
        I::Xor(_) => 4,
        I::Or(Register::Memory) | I::OrImmediate(_) => 7,
        I::Or(_) => 4,
        I::Compare(Register::Memory) | I::CompareImmediate(_) => 7,
        I::Compare(_) => 4,
        I::RotateLeft
        | I::RotateLeftThroughCarry
        | I::RotateRight
        | I::RotateRightThroughCarry
        | I::ComplementAccum
        | I::ComplementCarry
        | I::SetCarry => 4,
        I::Jump(_) | I::ConditionalJump(_, _) => 10,
        I::Call(_) => 17,
        I::ConditionalCall(condition, _) => {
            if state.condition_flags.is_condition_fulfilled(condition) {
                17
            } else {
                11
            }
        }
        I::Return => 10,
        I::ConditionalReturn(condition) => {
            if state.condition_flags.is_condition_fulfilled(condition) {
                11
            } else {
                5
            }
        }
        I::Restart(_) => 11,
        I::JumpHLIndirect => 5,
        I::PushRegPair(_) | I::PushPSW => 11,
        I::PopRegPair(_) | I::PopPSW => 10,
        I::ExchangeStackTopWithHL => 18,
        I::MoveHLToSP => 5,
        I::Input(_) | I::Output(_) => 10,
        I::EnableInterrupts | I::DisableInterrupts => 4,
        I::Halt => 7,
        I::NoOp => 4,
    }
}
