use crate::internal::{
    execution::execute_instruction,
    instructions::{timing::get_instruction_timing, Instruction},
    memory::AddressableMemory,
    state::State,
};

pub mod test;

pub struct System {
    state: State,
    interrupt_instruction: Option<Instruction>,
}

impl System {
    pub fn new() -> Self {
        System {
            state: State::new(),
            interrupt_instruction: None,
        }
    }

    pub fn load_program(&mut self, program_bytecode: Vec<u8>) {
        let address_end = program_bytecode.len() as u16;

        self.state
            .memory
            .set_range(0x00, address_end, program_bytecode);
    }

    pub fn read_memory_region(&self, address_start: u16, address_end: u16) -> Vec<u8> {
        self.state.memory.get_range(address_start, address_end)
    }

    pub fn run(&mut self, max_clock_cycles: usize) {
        let mut clock_cycles: usize = 0;

        while self.state.enabled && clock_cycles < max_clock_cycles {
            let instruction = self
                .state
                .program_counter
                .get_next_instruction(&self.state.memory);
            let instruction_cycles = get_instruction_timing(&self.state, &instruction);

            execute_instruction(&mut self.state, &instruction);

            clock_cycles += instruction_cycles;

            if let Some(interrupt_instruction) = &self.interrupt_instruction {
                self.state.interrupt_enabled = false;

                let interrupt_cycles = get_instruction_timing(&self.state, &interrupt_instruction);

                execute_instruction(&mut self.state, &interrupt_instruction);

                self.interrupt_instruction = None;
                clock_cycles += interrupt_cycles;
            }
        }
    }

    pub fn interrupt(&mut self, subroutine_address: u8) {
        self.interrupt_instruction = Some(Instruction::Restart(subroutine_address));
    }

    pub fn set_input(&mut self, port: u8, value: u8) {
        self.state.inputs.set(port, value);
    }

    pub fn get_output(&self, port: u8) -> u8 {
        self.state.outputs.get(port)
    }
}
