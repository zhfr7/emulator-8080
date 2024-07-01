use crate::{
    execution::execute_instruction,
    instructions::{
        parsers::parse_instruction, Instruction, MachineControlInstruction, Register, RegisterPair,
    },
    state::State,
};

pub use crate::memory::AddressableMemory;

pub struct TestSystem {
    pub state: State,
}

impl TestSystem {
    pub fn new() -> Self {
        let mut state = State::new();

        state.program_counter = 0x100;

        // Inject HLT at 0x00
        state.memory.set(0x00, 0x76);

        // Inject OUT 0; RET at 0x0005
        state.memory.set(0x0005, 0xD3);
        state.memory.set(0x0006, 0x00);
        state.memory.set(0x0007, 0xC9);

        TestSystem { state }
    }

    pub fn load_test_program(&mut self, program_bytecode: Vec<u8>) {
        let address_end = 0x100u16.wrapping_add(program_bytecode.len() as u16);

        self.state
            .memory
            .set_range(0x100, address_end, program_bytecode);
    }

    fn print(&mut self) {
        let operation = self.state.registers.get(&Register::C);

        match operation {
            2 => print!("{}", char::from(self.state.registers.get(&Register::D))),
            9 => {
                let mut address = self.state.registers.get_pair(&RegisterPair::DE);
                let mut value = self.state.memory.get(address);

                while value != b'$' {
                    print!("{}", char::from(value));

                    address = address.wrapping_add(1);
                    value = self.state.memory.get(address);
                }
            }
            _ => {}
        }
    }

    pub fn run_current_instruction(&mut self) {
        // Get the next two bytes after the program counter for parsing
        // since instructions can be 3-bytes long
        let end_address =
            self.state.program_counter + (u16::MAX - self.state.program_counter).min(2);

        let bytes = self
            .state
            .memory
            .get_range(self.state.program_counter, end_address);

        let instruction = parse_instruction(&bytes)
            .map(|(_, instruction)| instruction)
            .unwrap_or(Instruction::MachineControl(MachineControlInstruction::NoOp));

        execute_instruction(&mut self.state, &instruction);

        if let Instruction::MachineControl(MachineControlInstruction::Output(_)) = &instruction {
            self.print()
        }
    }
}
