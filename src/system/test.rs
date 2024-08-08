use crate::internal::{
    execution::execute_instruction,
    instructions::{Instruction, Register, RegisterPair},
    memory::AddressableMemory,
    state::State,
};

pub struct TestSystem {
    pub state: State,
}

impl TestSystem {
    pub fn new() -> Self {
        let mut state = State::new();

        state.program_counter.set(0x100);

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
        let operation = self.state.get_register(&Register::C);

        match operation {
            2 => print!("{}", char::from(self.state.get_register(&Register::E))),
            9 => {
                let mut address = self.state.get_register_pair(&RegisterPair::DE);
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
        // print!("{:#06x} ", self.state.program_counter.get());

        let instruction = self
            .state
            .program_counter
            .get_next_instruction(&self.state.memory);

        execute_instruction(&mut self.state, &instruction);

        if let Instruction::Output(_) = &instruction {
            self.print()
        }

        // println!("{}:   {:#04x}", &instruction, self.state.registers.a);
    }
}
