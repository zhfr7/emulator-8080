use crate::internal::{
    instructions::{Register, RegisterPair},
    state::State,
};

fn add(x: u8, y: u8, carry: bool) -> (u8, bool, bool) {
    let carry_bit = if carry { 1 } else { 0 };
    let result = (x as u16) + (y as u16) + carry_bit;
    let carry = result >> 8 != 0;
    let aux_carry = ((x & 0x0F) + (y & 0x0F) + carry_bit as u8) >> 4 != 0;

    (result.to_be_bytes()[1], carry, aux_carry)
}

pub fn sub(x: u8, y: u8, carry: bool) -> (u8, bool, bool) {
    let (result, c, aux_carry) = add(x, !y, !carry);

    (result, !c, aux_carry)
}

pub fn execute_add(state: &mut State, value: u8) {
    let accum_value = state.get_register(&Register::A);

    let (result, carry, aux_carry) = add(accum_value, value, false);

    state.set_register(&Register::A, result);
    state.condition_flags.set_zero_sign_parity_flags(result);
    state.condition_flags.carry = carry;
    state.condition_flags.aux_carry = aux_carry;
}

pub fn execute_add_with_carry(state: &mut State, value: u8) {
    let accum_value = state.get_register(&Register::A);

    let (result, carry, aux_carry) = add(accum_value, value, state.condition_flags.carry);

    state.set_register(&Register::A, result);
    state.condition_flags.set_zero_sign_parity_flags(result);
    state.condition_flags.carry = carry;
    state.condition_flags.aux_carry = aux_carry;
}

pub fn execute_subtract(state: &mut State, value: u8) {
    let accum_value = state.get_register(&Register::A);

    let (result, carry, aux_carry) = sub(accum_value, value, false);

    state.set_register(&Register::A, result);
    state.condition_flags.set_zero_sign_parity_flags(result);
    state.condition_flags.carry = carry;
    state.condition_flags.aux_carry = aux_carry;
}

pub fn execute_subtract_with_borrow(state: &mut State, value: u8) {
    let accum_value = state.get_register(&Register::A);

    let (result, carry, aux_carry) = sub(accum_value, value, state.condition_flags.carry);

    state.set_register(&Register::A, result);
    state.condition_flags.set_zero_sign_parity_flags(result);
    state.condition_flags.carry = carry;
    state.condition_flags.aux_carry = aux_carry;
}

pub fn execute_increment(state: &mut State, register: &Register) {
    let value = state.get_register(register);
    let (result, _, aux_carry) = add(value, 1, false);

    state.set_register(register, result);
    state.condition_flags.set_zero_sign_parity_flags(result);
    state.condition_flags.aux_carry = aux_carry;
}

pub fn execute_decrement(state: &mut State, register: &Register) {
    let value = state.get_register(register);
    let (result, _, aux_carry) = sub(value, 1, false);

    state.set_register(register, result);
    state.condition_flags.set_zero_sign_parity_flags(result);
    state.condition_flags.aux_carry = aux_carry;
}

pub fn execute_increment_reg_pair(state: &mut State, register_pair: &RegisterPair) {
    let value = state.get_register_pair(register_pair);

    state.set_register_pair(register_pair, value.wrapping_add(1));
}

pub fn execute_decrement_reg_pair(state: &mut State, register_pair: &RegisterPair) {
    let value = state.get_register_pair(register_pair);

    state.set_register_pair(register_pair, value.wrapping_sub(1));
}

pub fn execute_add_reg_pair_to_hl(state: &mut State, register_pair: &RegisterPair) {
    let hl_value = state.get_register_pair(&RegisterPair::HL);
    let value = state.get_register_pair(&register_pair);

    let (result, carry) = hl_value.overflowing_add(value);

    state.set_register_pair(&RegisterPair::HL, result);
    state.condition_flags.carry = carry;
}

pub fn execute_decimal_adjust(state: &mut State) {
    let mut correction = 0u8;
    let mut carry = false;

    let value = state.get_register(&Register::A);
    let low_bits = value & 0x0F;
    let high_bits = value >> 4;

    if state.condition_flags.aux_carry || low_bits > 9 {
        correction += 0x06;
    }

    if state.condition_flags.carry || high_bits > 9 || (high_bits >= 9 && low_bits > 9) {
        correction += 0x60;
        carry = true;
    }

    let (result, _, aux_carry) = add(value, correction, false);
    state.set_register(&Register::A, result);
    state.condition_flags.set_zero_sign_parity_flags(result);
    state.condition_flags.carry = carry;
    state.condition_flags.aux_carry = aux_carry;
}
