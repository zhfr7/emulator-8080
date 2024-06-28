use crate::{
    instructions::{LogicalInstruction as LI, Register},
    state::State,
};

use super::sub;

fn update_state_from_value(state: &mut State, value: u8) {
    state.registers.set(&Register::A, value);
    state.registers.set_zero_sign_parity_flags(value);
    state.registers.set_carry_flag(false);
    state.registers.set_carry_flag(false);
}

pub fn execute_logical_instruction(state: &mut State, instruction: &LI) {
    match instruction {
        LI::And(register) => {
            let value = state.registers.get(register);
            let accum_value = state.registers.get(&Register::A);

            update_state_from_value(state, value & accum_value);
        }
        LI::AndMem => {
            let value = state.get_memory_value();
            let accum_value = state.registers.get(&Register::A);

            update_state_from_value(state, value & accum_value);
        }
        LI::AndImmediate(value) => {
            let accum_value = state.registers.get(&Register::A);

            update_state_from_value(state, value & accum_value);
        }
        LI::Xor(register) => {
            let value = state.registers.get(register);
            let accum_value = state.registers.get(&Register::A);

            update_state_from_value(state, value ^ accum_value)
        }
        LI::XorMem => {
            let value = state.get_memory_value();
            let accum_value = state.registers.get(&Register::A);

            update_state_from_value(state, value ^ accum_value);
        }
        LI::XorImmediate(value) => {
            let accum_value = state.registers.get(&Register::A);

            update_state_from_value(state, value ^ accum_value);
        }
        LI::Or(register) => {
            let value = state.registers.get(register);
            let accum_value = state.registers.get(&Register::A);

            update_state_from_value(state, value | accum_value);
        }
        LI::OrMem => {
            let value = state.get_memory_value();
            let accum_value = state.registers.get(&Register::A);

            update_state_from_value(state, value | accum_value);
        }
        LI::OrImmediate(value) => {
            let accum_value = state.registers.get(&Register::A);

            update_state_from_value(state, value | accum_value);
        }
        LI::Compare(register) => {
            let value = state.registers.get(register);
            let accum_value = state.registers.get(&Register::A);

            let (difference, carry, aux_carry) = sub(accum_value, value);

            state.registers.set_zero_sign_parity_flags(difference);
            state.registers.set_carry_flag(carry);
            state.registers.set_aux_carry_flag(aux_carry);
        }
        LI::CompareMem => {
            let value = state.get_memory_value();
            let accum_value = state.registers.get(&Register::A);

            let (difference, carry, aux_carry) = sub(accum_value, value);

            state.registers.set_zero_sign_parity_flags(difference);
            state.registers.set_carry_flag(carry);
            state.registers.set_aux_carry_flag(aux_carry);
        }
        LI::CompareImmediate(value) => {
            let accum_value = state.registers.get(&Register::A);

            let (difference, carry, aux_carry) = sub(accum_value, *value);

            state.registers.set_zero_sign_parity_flags(difference);
            state.registers.set_carry_flag(carry);
            state.registers.set_aux_carry_flag(aux_carry);
        }
        LI::RotateLeft => {
            let accum_value = state.registers.get(&Register::A);
            let carry = accum_value & 0x80 > 0;

            state
                .registers
                .set(&Register::A, accum_value.rotate_left(1));
            state.registers.set_carry_flag(carry);
        }
        LI::RotateRight => {
            let accum_value = state.registers.get(&Register::A);
            let carry = accum_value & 0x01 > 0;

            state
                .registers
                .set(&Register::A, accum_value.rotate_right(1));
            state.registers.set_carry_flag(carry);
        }
        LI::RotateLeftThroughCarry => {
            let accum_value = state.registers.get(&Register::A);
            let carry = accum_value & 0x80 > 0;

            let value = accum_value.rotate_left(1)
                | if state.registers.condition_flags.carry {
                    0x01
                } else {
                    0
                };

            state.registers.set(&Register::A, value);
            state.registers.set_carry_flag(carry);
        }
        LI::RotateRightThroughCarry => {
            let accum_value = state.registers.get(&Register::A);
            let carry = accum_value & 0x01 > 0;

            let value = accum_value.rotate_right(1)
                | if state.registers.condition_flags.carry {
                    0x80
                } else {
                    0
                };

            state.registers.set(&Register::A, value);
            state.registers.set_carry_flag(carry);
        }
        LI::ComplementAccum => {
            let accum_value = state.registers.get(&Register::A);

            state.registers.set(&Register::A, !accum_value);
        }
        LI::ComplementCarry => {
            state
                .registers
                .set_carry_flag(!state.registers.condition_flags.carry);
        }
        LI::SetCarry => {
            state.registers.set_carry_flag(true);
        }
    }

    state.increment_program_counter();
}
