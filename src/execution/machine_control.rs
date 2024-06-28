use crate::{
    instructions::{MachineControlInstruction as MCI, Register, RegisterPair},
    memory::AddressableMemory,
    state::State,
};

pub fn execute_machine_control_instruction(state: &mut State, instruction: &MCI) {
    match instruction {
        MCI::PushRegPair(register_pair) => {
            let value = state.registers.get_pair(register_pair);

            state.push_word_to_stack(value);
        }
        MCI::PushPSW => {
            let psw = state.registers.get_psw();

            state.push_word_to_stack(psw);
        }
        MCI::PopRegPair(register_pair) => {
            let value = state.pop_word_from_stack();

            state.registers.set_pair(register_pair, value);
        }
        MCI::PopPSW => {
            let psw = state.pop_word_from_stack();

            state.registers.set_psw(psw);
        }
        MCI::ExchangeStackTopWithHL => {
            let l_address = state.registers.stack_pointer;
            let h_address = state.registers.stack_pointer.wrapping_add(1);

            let l_byte = state.memory.get(l_address);
            let h_byte = state.memory.get(h_address);

            state
                .memory
                .set(l_address, state.registers.get(&Register::L));
            state
                .memory
                .set(h_address, state.registers.get(&Register::L));

            state.registers.set(&Register::L, l_byte);
            state.registers.set(&Register::H, h_byte);
        }
        MCI::MoveHLToSP => {
            let hl = state.registers.get_pair(&RegisterPair::HL);

            state.registers.stack_pointer = hl;
        }
        MCI::Input(port) => {
            let input_value = state.inputs.get(*port);

            state.registers.set(&Register::A, input_value);
        }
        MCI::Output(port) => {
            let output_value = state.registers.get(&Register::A);

            state.outputs.set(*port, output_value);
        }
        MCI::EnableInterrupts => {
            state.interrupt_enabled = true;
        }
        MCI::DisableInterrupts => {
            state.interrupt_enabled = false;
        }
        MCI::Halt => {
            state.enabled = false;
        }
        MCI::NoOp => {}
    }

    state.increment_program_counter();
}
