use crate::internal::state::State;

pub fn execute_jump(state: &mut State, address: u16, condition: bool) {
    if condition {
        state.program_counter.set(address);
    }
}

pub fn execute_call(state: &mut State, address: u16, condition: bool) {
    if condition {
        state.push_word_to_stack(state.program_counter.get());
        state.program_counter.set(address);
    }
}

pub fn execute_return(state: &mut State, condition: bool) {
    if condition {
        let address = state.pop_word_from_stack();
        state.program_counter.set(address);
    }
}

pub fn execute_restart(state: &mut State, n: u8) {
    state.push_word_to_stack(state.program_counter.get());
    state.program_counter.set((n as u16) << 3);
}
