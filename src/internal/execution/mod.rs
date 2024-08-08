use arithmetic::{
    execute_add, execute_add_reg_pair_to_hl, execute_add_with_carry, execute_decimal_adjust,
    execute_decrement, execute_decrement_reg_pair, execute_increment, execute_increment_reg_pair,
    execute_subtract, execute_subtract_with_borrow,
};
use branch::{execute_call, execute_jump, execute_restart, execute_return};
use data_transfer::{
    execute_exchange_hl_with_de, execute_load_accum_direct, execute_load_accum_indirect,
    execute_load_hl_direct, execute_load_reg_pair_immediate, execute_move,
    execute_store_accum_direct, execute_store_accum_indirect, execute_store_hl_direct,
};
use logical::{
    execute_and, execute_compare, execute_complement_accum, execute_complement_carry, execute_or,
    execute_rotate_left, execute_rotate_left_through_carry, execute_rotate_right,
    execute_rotate_right_through_carry, execute_set_carry, execute_xor,
};
use machine_control::{
    execute_exchange_stack_top_with_hl, execute_input, execute_move_hl_to_sp, execute_output,
    execute_pop_psw, execute_pop_reg_pair, execute_push_stack, execute_set_interrupt,
};

use super::{
    instructions::{Instruction, RegisterPair},
    state::State,
};

mod arithmetic;
mod branch;
mod data_transfer;
mod logical;
mod machine_control;

pub fn execute_instruction(state: &mut State, instruction: &Instruction) {
    match instruction {
        // Data transfer
        Instruction::Move(source, destination) => {
            execute_move(state, destination, state.get_register(source))
        }
        Instruction::MoveImmediate(destination, value) => execute_move(state, destination, *value),
        Instruction::LoadRegisterPairImmediate(register_pair, value) => {
            execute_load_reg_pair_immediate(state, register_pair, *value)
        }
        Instruction::LoadAccumDirect(address) => execute_load_accum_direct(state, *address),
        Instruction::StoreAccumDirect(address) => execute_store_accum_direct(state, *address),
        Instruction::LoadHLDirect(address) => execute_load_hl_direct(state, *address),
        Instruction::StoreHLDirect(address) => execute_store_hl_direct(state, *address),
        Instruction::LoadAccumIndirect(register_pair) => {
            execute_load_accum_indirect(state, register_pair)
        }
        Instruction::StoreAccumIndirect(register_pair) => {
            execute_store_accum_indirect(state, register_pair)
        }
        Instruction::ExchangeHLWithDE => execute_exchange_hl_with_de(state),

        // Arithmetic
        Instruction::Add(register) => execute_add(state, state.get_register(register)),
        Instruction::AddImmediate(value) => execute_add(state, *value),
        Instruction::AddWithCarry(register) => {
            execute_add_with_carry(state, state.get_register(register))
        }
        Instruction::AddImmediateWithCarry(value) => execute_add_with_carry(state, *value),
        Instruction::Subtract(register) => execute_subtract(state, state.get_register(register)),
        Instruction::SubtractImmediate(value) => execute_subtract(state, *value),
        Instruction::SubtractWithBorrow(register) => {
            execute_subtract_with_borrow(state, state.get_register(register))
        }
        Instruction::SubtractImmediateWithBorrow(value) => {
            execute_subtract_with_borrow(state, *value)
        }
        Instruction::Increment(register) => execute_increment(state, register),
        Instruction::Decrement(register) => execute_decrement(state, register),
        Instruction::IncrementRegPair(register_pair) => {
            execute_increment_reg_pair(state, register_pair)
        }
        Instruction::DecrementRegPair(register_pair) => {
            execute_decrement_reg_pair(state, register_pair)
        }
        Instruction::AddRegPairToHL(register_pair) => {
            execute_add_reg_pair_to_hl(state, register_pair)
        }
        Instruction::DecimalAdjustAccum => {
            execute_decimal_adjust(state);
        }

        // Logical
        Instruction::And(register) => execute_and(state, state.get_register(register)),
        Instruction::AndImmediate(value) => execute_and(state, *value),
        Instruction::Xor(register) => execute_xor(state, state.get_register(register)),
        Instruction::XorImmediate(value) => execute_xor(state, *value),
        Instruction::Or(register) => execute_or(state, state.get_register(register)),
        Instruction::OrImmediate(value) => execute_or(state, *value),
        Instruction::Compare(register) => execute_compare(state, state.get_register(register)),
        Instruction::CompareImmediate(value) => execute_compare(state, *value),
        Instruction::RotateLeft => execute_rotate_left(state),
        Instruction::RotateRight => execute_rotate_right(state),
        Instruction::RotateLeftThroughCarry => execute_rotate_left_through_carry(state),
        Instruction::RotateRightThroughCarry => execute_rotate_right_through_carry(state),
        Instruction::ComplementAccum => execute_complement_accum(state),
        Instruction::ComplementCarry => execute_complement_carry(state),
        Instruction::SetCarry => execute_set_carry(state),

        // Branch
        Instruction::Jump(address) => execute_jump(state, *address, true),
        Instruction::ConditionalJump(condition, address) => execute_jump(
            state,
            *address,
            state.condition_flags.is_condition_fulfilled(condition),
        ),
        Instruction::Call(address) => execute_call(state, *address, true),
        Instruction::ConditionalCall(condition, address) => execute_call(
            state,
            *address,
            state.condition_flags.is_condition_fulfilled(condition),
        ),
        Instruction::Return => execute_return(state, true),
        Instruction::ConditionalReturn(condition) => execute_return(
            state,
            state.condition_flags.is_condition_fulfilled(condition),
        ),
        Instruction::Restart(n) => execute_restart(state, *n),
        Instruction::JumpHLIndirect => {
            execute_jump(state, state.get_register_pair(&RegisterPair::HL), true)
        }

        // Machine control
        Instruction::PushRegPair(register_pair) => {
            execute_push_stack(state, state.get_register_pair(register_pair))
        }
        Instruction::PushPSW => execute_push_stack(state, state.get_psw()),
        Instruction::PopRegPair(register_pair) => execute_pop_reg_pair(state, register_pair),
        Instruction::PopPSW => execute_pop_psw(state),
        Instruction::ExchangeStackTopWithHL => execute_exchange_stack_top_with_hl(state),
        Instruction::MoveHLToSP => execute_move_hl_to_sp(state),
        Instruction::Input(port) => execute_input(state, *port),
        Instruction::Output(port) => execute_output(state, *port),
        Instruction::EnableInterrupts => execute_set_interrupt(state, true),
        Instruction::DisableInterrupts => execute_set_interrupt(state, false),
        Instruction::Halt => state.enabled = false,
        Instruction::NoOp => {}
    }
}
