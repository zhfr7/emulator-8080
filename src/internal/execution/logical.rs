use crate::internal::{instructions::Register, state::State};

use super::arithmetic::sub;

fn update_state_from_value(state: &mut State, value: u8) {
    state.set_register(&Register::A, value);
    state.condition_flags.set_zero_sign_parity_flags(value);
    state.condition_flags.carry = false;
}

pub fn execute_and(state: &mut State, value: u8) {
    let accum_value = state.get_register(&Register::A);

    update_state_from_value(state, value & accum_value);
    state.condition_flags.aux_carry = ((accum_value | value) & 0x08) != 0;
}

pub fn execute_xor(state: &mut State, value: u8) {
    let accum_value = state.get_register(&Register::A);

    update_state_from_value(state, value ^ accum_value);
    state.condition_flags.aux_carry = false;
}

pub fn execute_or(state: &mut State, value: u8) {
    let accum_value = state.get_register(&Register::A);

    update_state_from_value(state, value | accum_value);
    state.condition_flags.aux_carry = false;
}

pub fn execute_compare(state: &mut State, value: u8) {
    let accum_value = state.get_register(&Register::A);

    let (difference, carry, aux_carry) = sub(accum_value, value, false);

    state.condition_flags.set_zero_sign_parity_flags(difference);
    state.condition_flags.carry = carry;
    state.condition_flags.aux_carry = aux_carry;
}

pub fn execute_rotate_left(state: &mut State) {
    let accum_value = state.get_register(&Register::A);
    let carry = accum_value & 0x80 != 0;

    state.set_register(&Register::A, accum_value.rotate_left(1));
    state.condition_flags.carry = carry;
}

pub fn execute_rotate_right(state: &mut State) {
    let accum_value = state.get_register(&Register::A);
    let carry = accum_value & 0x01 != 0;

    state.set_register(&Register::A, accum_value.rotate_right(1));
    state.condition_flags.carry = carry;
}

pub fn execute_rotate_left_through_carry(state: &mut State) {
    let accum_value = state.get_register(&Register::A);
    let carry = accum_value & 0x80 != 0;

    let value = accum_value << 1 | if state.condition_flags.carry { 0x01 } else { 0 };

    state.set_register(&Register::A, value);
    state.condition_flags.carry = carry;
}

pub fn execute_rotate_right_through_carry(state: &mut State) {
    let accum_value = state.get_register(&Register::A);
    let carry = accum_value & 0x01 != 0;

    let value = accum_value >> 1 | if state.condition_flags.carry { 0x80 } else { 0 };

    state.set_register(&Register::A, value);
    state.condition_flags.carry = carry;
}

pub fn execute_complement_accum(state: &mut State) {
    let accum_value = state.get_register(&Register::A);

    state.set_register(&Register::A, !accum_value);
}

pub fn execute_complement_carry(state: &mut State) {
    state.condition_flags.carry = !state.condition_flags.carry;
}

pub fn execute_set_carry(state: &mut State) {
    state.condition_flags.carry = true;
}
