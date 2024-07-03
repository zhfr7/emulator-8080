use crate::{
    instructions::{ArithmeticInstruction as AI, Register, RegisterPair},
    state::State,
};

use super::{add, add_with_carry, sub, sub_with_borrow};

fn decimal_adjust(value: u8, carry: bool, aux_carry: bool) -> (u8, bool, bool) {
    let mut result = value;
    let mut result_aux_carry = false;
    let mut result_carry = carry;
    let low_bits = value & 0x0F;

    if low_bits > 9 || aux_carry {
        result = result.wrapping_add(6);
        result_aux_carry = result >> 4 != value >> 4;
    }

    let high_bits = result >> 4;

    if high_bits > 9 || carry {
        let (r, c) = result.overflowing_add(0x60);

        result = r;
        result_carry = c;
    }

    (result, result_carry, result_aux_carry)
}

fn update_state_from_operation(state: &mut State, (result, carry, aux_carry): (u8, bool, bool)) {
    state.registers.set(&Register::A, result);
    state.registers.set_zero_sign_parity_flags(result);
    state.registers.set_carry_flag(carry);
    state.registers.set_aux_carry_flag(aux_carry);
}

pub fn execute_arithmetic_instruction(state: &mut State, instruction: &AI) {
    match instruction {
        AI::Add(register) => {
            let register_value = state.registers.get(register);
            let accum_value = state.registers.get(&Register::A);

            let op_result = add(register_value, accum_value);

            update_state_from_operation(state, op_result);
        }
        AI::AddMem => {
            let value = state.get_memory_value();
            let accum_value = state.registers.get(&Register::A);

            let op_result = add(accum_value, value);

            update_state_from_operation(state, op_result);
        }
        AI::AddImmediate(data) => {
            let accum_value = state.registers.get(&Register::A);

            let op_result = add(accum_value, *data);

            update_state_from_operation(state, op_result);

            state.increment_program_counter();
        }
        AI::AddWithCarry(register) => {
            let register_value = state.registers.get(register);
            let accum_value = state.registers.get(&Register::A);

            let op_result = add_with_carry(
                register_value,
                accum_value,
                state.registers.condition_flags.carry,
            );

            update_state_from_operation(state, op_result);
        }
        AI::AddMemWithCarry => {
            let value = state.get_memory_value();
            let accum_value = state.registers.get(&Register::A);

            let op_result =
                add_with_carry(value, accum_value, state.registers.condition_flags.carry);

            update_state_from_operation(state, op_result);
        }
        AI::AddImmediateWithCarry(data) => {
            let accum_value = state.registers.get(&Register::A);

            let op_result =
                add_with_carry(*data, accum_value, state.registers.condition_flags.carry);

            update_state_from_operation(state, op_result);

            state.increment_program_counter();
        }
        AI::Subtract(register) => {
            let register_value = state.registers.get(register);
            let accum_value = state.registers.get(&Register::A);

            let op_result = sub(accum_value, register_value);

            update_state_from_operation(state, op_result);
        }
        AI::SubtractMem => {
            let value = state.get_memory_value();
            let accum_value = state.registers.get(&Register::A);

            let op_result = sub(accum_value, value);

            update_state_from_operation(state, op_result);
        }
        AI::SubtractImmediate(data) => {
            let accum_value = state.registers.get(&Register::A);

            let op_result = sub(accum_value, *data);

            update_state_from_operation(state, op_result);

            state.increment_program_counter();
        }
        AI::SubtractWithBorrow(register) => {
            let register_value = state.registers.get(register);
            let accum_value = state.registers.get(&Register::A);

            let op_result = sub_with_borrow(
                accum_value,
                register_value,
                state.registers.condition_flags.carry,
            );

            update_state_from_operation(state, op_result);
        }
        AI::SubtractMemWithBorrow => {
            let value = state.get_memory_value();
            let accum_value = state.registers.get(&Register::A);

            let op_result =
                sub_with_borrow(accum_value, value, state.registers.condition_flags.carry);

            update_state_from_operation(state, op_result);
        }
        AI::SubtractImmediateWithBorrow(data) => {
            let accum_value = state.registers.get(&Register::A);

            let op_result =
                sub_with_borrow(accum_value, *data, state.registers.condition_flags.carry);

            update_state_from_operation(state, op_result);

            state.increment_program_counter();
        }
        AI::Increment(register) => {
            let value = state.registers.get(register);
            let (result, _, aux_carry) = add(value, 1);

            state.registers.set(&register, result);
            state.registers.set_zero_sign_parity_flags(result);
            state.registers.set_aux_carry_flag(aux_carry);
        }
        AI::IncrementMem => {
            let value = state.get_memory_value();
            let (result, _, aux_carry) = add(value, 1);

            state.set_memory_value(result);
            state.registers.set_zero_sign_parity_flags(result);
            state.registers.set_aux_carry_flag(aux_carry);
        }
        AI::Decrement(register) => {
            let value = state.registers.get(register);
            let (result, _, aux_carry) = sub(value, 1);

            state.registers.set(&register, result);
            state.registers.set_zero_sign_parity_flags(result);
            state.registers.set_aux_carry_flag(aux_carry);
        }
        AI::DecrementMem => {
            let value = state.get_memory_value();
            let (result, _, aux_carry) = sub(value, 1);

            state.set_memory_value(result);
            state.registers.set_zero_sign_parity_flags(result);
            state.registers.set_aux_carry_flag(aux_carry);
        }
        AI::IncrementRegPair(register_pair) => {
            let value = state.registers.get_pair(&register_pair);

            state
                .registers
                .set_pair(&register_pair, value.wrapping_add(1));
        }
        AI::DecrementRegPair(register_pair) => {
            let value = state.registers.get_pair(&register_pair);

            state
                .registers
                .set_pair(&register_pair, value.wrapping_sub(1));
        }
        AI::AddRegPairToHL(register_pair) => {
            let hl_value = state.registers.get_pair(&RegisterPair::HL);
            let value = state.registers.get_pair(&register_pair);

            let (result, carry) = hl_value.overflowing_add(value);

            state.registers.set_pair(&RegisterPair::HL, result);
            state.registers.set_carry_flag(carry);
        }
        AI::DecimalAdjustAccum => {
            let value = state.registers.get(&Register::A);

            let (result, carry, aux_carry) = decimal_adjust(
                value,
                state.registers.condition_flags.carry,
                state.registers.condition_flags.aux_carry,
            );

            update_state_from_operation(state, (result, carry, aux_carry));
        }
    }

    state.increment_program_counter();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_decimal_adjust_correctly() {
        let (r, c, ac) = decimal_adjust(0x9b, false, false);

        assert_eq!(r, 0x01);
        assert!(c);
        assert!(ac);
    }
}
