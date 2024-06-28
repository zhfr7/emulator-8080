use crate::{
    execution::execute_instruction,
    instructions::{
        parsers::parse_instruction, timing::get_instruction_timing, BranchInstruction, Instruction,
        MachineControlInstruction,
    },
    memory::AddressableMemory,
    state::State,
};

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
        self.load_program_at(program_bytecode, 0);
    }

    pub fn load_program_at(&mut self, program_bytecode: Vec<u8>, start: u16) {
        let address_end = start.wrapping_add(program_bytecode.len() as u16);

        self.state
            .memory
            .set_range(start, address_end, program_bytecode);
    }

    pub fn read_memory_region(&self, address_start: u16, address_end: u16) -> Vec<u8> {
        self.state.memory.get_range(address_start, address_end)
    }

    pub fn set_program_counter(&mut self, address: u16) {
        self.state.program_counter = address;
    }

    fn get_current_instruction(&self) -> Instruction {
        // Get the next two bytes after the program counter for parsing
        // since instructions can be 3-bytes long
        let end_address =
            self.state.program_counter + (u16::MAX - self.state.program_counter).min(2);

        let bytes = self.read_memory_region(self.state.program_counter, end_address);

        parse_instruction(&bytes)
            .map(|(_, instruction)| instruction)
            .unwrap_or(Instruction::MachineControl(MachineControlInstruction::NoOp))
    }

    pub fn run(&mut self, max_clock_cycles: usize) {
        let mut clock_cycles: usize = 0;

        while self.state.enabled && clock_cycles < max_clock_cycles {
            let instruction = self.get_current_instruction();
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
        self.interrupt_instruction = Some(Instruction::Branch(BranchInstruction::Restart(
            subroutine_address,
        )));
    }

    pub fn set_input(&mut self, port: u8, value: u8) {
        self.state.inputs.set(port, value);
    }

    pub fn get_output(&self, port: u8) -> u8 {
        self.state.outputs.get(port)
    }
}
