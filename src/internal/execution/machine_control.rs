use crate::internal::{
    instructions::{Register, RegisterPair},
    memory::AddressableMemory,
    state::State,
};

pub fn execute_push_stack(state: &mut State, value: u16) {
    state.push_word_to_stack(value)
}

pub fn execute_pop_reg_pair(state: &mut State, register_pair: &RegisterPair) {
    let value = state.pop_word_from_stack();

    state.set_register_pair(register_pair, value);
}

pub fn execute_pop_psw(state: &mut State) {
    let value = state.pop_word_from_stack();

    state.set_psw(value);
}

pub fn execute_exchange_stack_top_with_hl(state: &mut State) {
    let l_address = state.registers.stack_pointer;
    let h_address = state.registers.stack_pointer.wrapping_add(1);

    let l_byte = state.memory.get(l_address);
    let h_byte = state.memory.get(h_address);

    state
        .memory
        .set(l_address, state.get_register(&Register::L));
    state
        .memory
        .set(h_address, state.get_register(&Register::H));

    state.set_register(&Register::L, l_byte);
    state.set_register(&Register::H, h_byte);
}

pub fn execute_move_hl_to_sp(state: &mut State) {
    let hl = state.get_register_pair(&RegisterPair::HL);

    state.registers.stack_pointer = hl;
}

pub fn execute_input(state: &mut State, port: u8) {
    let input_value = state.inputs.get(port);

    state.set_register(&Register::A, input_value);
}

pub fn execute_output(state: &mut State, port: u8) {
    let output_value = state.get_register(&Register::A);

    state.outputs.set(port, output_value);
}

pub fn execute_set_interrupt(state: &mut State, value: bool) {
    state.interrupt_enabled = value
}
