use crate::internal::{
    instructions::{Register, RegisterPair},
    memory::AddressableMemory,
    state::State,
};

pub fn execute_move(state: &mut State, destination: &Register, value: u8) {
    state.set_register(destination, value);
}

pub fn execute_load_reg_pair_immediate(
    state: &mut State,
    register_pair: &RegisterPair,
    value: u16,
) {
    state.set_register_pair(register_pair, value);
}

pub fn execute_load_accum_direct(state: &mut State, address: u16) {
    let value = state.memory.get(address);

    state.set_register(&Register::A, value);
}

pub fn execute_store_accum_direct(state: &mut State, address: u16) {
    let value = state.get_register(&Register::A);

    state.memory.set(address, value);
}

pub fn execute_load_hl_direct(state: &mut State, address: u16) {
    let succeeding_address = address.wrapping_add(1);
    let l_value = state.memory.get(address);
    let h_value = state.memory.get(succeeding_address);

    state.set_register(&Register::L, l_value);
    state.set_register(&Register::H, h_value);
}

pub fn execute_store_hl_direct(state: &mut State, address: u16) {
    let succeeding_address = address.wrapping_add(1);
    let l_value = state.get_register(&Register::L);
    let h_value = state.get_register(&Register::H);

    state.memory.set(address, l_value);
    state.memory.set(succeeding_address, h_value);
}

pub fn execute_load_accum_indirect(state: &mut State, register_pair: &RegisterPair) {
    let address = state.get_register_pair(register_pair);
    let value = state.memory.get(address);

    state.set_register(&Register::A, value);
}

pub fn execute_store_accum_indirect(state: &mut State, register_pair: &RegisterPair) {
    let address = state.get_register_pair(register_pair);
    let value = state.get_register(&Register::A);

    state.memory.set(address, value);
}

pub fn execute_exchange_hl_with_de(state: &mut State) {
    let hl_value = state.get_register_pair(&RegisterPair::HL);

    state.set_register_pair(
        &RegisterPair::HL,
        state.get_register_pair(&RegisterPair::DE),
    );
    state.set_register_pair(&RegisterPair::DE, hl_value);
}
