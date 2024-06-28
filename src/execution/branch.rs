use crate::{
    instructions::{BranchInstruction as BI, Register},
    state::State,
};

pub fn execute_branch_instruction(state: &mut State, instruction: &BI) {
    match instruction {
        BI::Jump(address) => state.program_counter = *address,
        BI::ConditionalJump(condition, address) => {
            if state.registers.is_condition_fulfilled(condition) {
                state.program_counter = *address;
            } else {
                state.increment_program_counter_by(3);
            }
        }
        BI::Call(address) => {
            let next_instruction_address = state.program_counter.wrapping_add(3);

            state.push_word_to_stack(next_instruction_address);

            state.program_counter = *address;
        }
        BI::ConditionalCall(condition, address) => {
            let next_instruction_address = state.program_counter.wrapping_add(3);

            if state.registers.is_condition_fulfilled(condition) {
                state.push_word_to_stack(next_instruction_address);

                state.program_counter = *address;
            } else {
                state.program_counter = next_instruction_address;
            }
        }
        BI::Return => {
            let address = state.pop_word_from_stack();

            state.program_counter = address;
        }
        BI::ConditionalReturn(condition) => {
            if state.registers.is_condition_fulfilled(condition) {
                let address = state.pop_word_from_stack();

                state.program_counter = address;
            } else {
                state.increment_program_counter();
            }
        }
        BI::Restart(n) => {
            let next_instruction_address = state.program_counter.wrapping_add(1);

            state.push_word_to_stack(next_instruction_address);

            state.program_counter = (*n as u16) << 3;
        }
        BI::JumpHLIndirect => {
            let low_byte = state.registers.get(&Register::L);
            let high_byte = state.registers.get(&Register::H);

            state.program_counter = u16::from_be_bytes([high_byte, low_byte]);
        }
    }
}
