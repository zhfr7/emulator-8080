use crate::{
    memory::{internal::InternalMemory, io::IOMemory, AddressableMemory},
    register::Registers,
};

#[derive(Debug)]
pub struct State {
    pub enabled: bool,
    pub interrupt_enabled: bool,
    pub program_counter: u16,
    pub registers: Registers,
    pub memory: InternalMemory,
    pub inputs: IOMemory,
    pub outputs: IOMemory,
}

impl State {
    pub fn new() -> Self {
        Self {
            enabled: false,
            interrupt_enabled: true,
            program_counter: 0,
            registers: Registers::default(),
            memory: InternalMemory::new(),
            inputs: IOMemory::new(),
            outputs: IOMemory::new(),
        }
    }

    pub fn increment_program_counter(&mut self) {
        self.program_counter = self.program_counter.wrapping_add(1);
    }

    pub fn increment_program_counter_by(&mut self, offset: u16) {
        self.program_counter = self.program_counter.wrapping_add(offset);
    }

    pub fn get_memory_value(&self) -> u8 {
        let address = self.registers.get_memory_address();

        self.memory.get(address)
    }

    pub fn set_memory_value(&mut self, value: u8) {
        let address = self.registers.get_memory_address();

        self.memory.set(address, value)
    }

    pub fn push_word_to_stack(&mut self, value: u16) {
        let [high_byte, low_byte] = value.to_be_bytes();

        let high_address = self.registers.stack_pointer.wrapping_sub(1);
        let low_address = self.registers.stack_pointer.wrapping_sub(2);

        self.memory.set(high_address, high_byte);
        self.memory.set(low_address, low_byte);

        self.registers.stack_pointer = self.registers.stack_pointer.wrapping_sub(2);
    }

    pub fn pop_word_from_stack(&mut self) -> u16 {
        let low_byte = self.memory.get(self.registers.stack_pointer);
        let high_byte = self
            .memory
            .get(self.registers.stack_pointer.wrapping_add(1));

        self.registers.stack_pointer = self.registers.stack_pointer.wrapping_add(2);

        u16::from_be_bytes([high_byte, low_byte])
    }
}
