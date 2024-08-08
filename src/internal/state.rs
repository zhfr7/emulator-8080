use crate::internal::instructions::{Register, RegisterPair};

use super::{
    condition_flags::ConditionFlags,
    memory::{internal::InternalMemory, io::IOMemory, AddressableMemory},
    program_counter::ProgramCounter,
    register::Registers,
};

#[derive(Debug)]
pub struct State {
    pub enabled: bool,
    pub interrupt_enabled: bool,
    pub program_counter: ProgramCounter,
    pub registers: Registers,
    pub condition_flags: ConditionFlags,
    pub memory: InternalMemory,
    pub inputs: IOMemory,
    pub outputs: IOMemory,
}

impl State {
    pub fn new() -> Self {
        Self {
            enabled: true,
            interrupt_enabled: true,
            program_counter: ProgramCounter::new(),
            registers: Registers::default(),
            condition_flags: ConditionFlags::default(),
            memory: InternalMemory::new(),
            inputs: IOMemory::new(),
            outputs: IOMemory::new(),
        }
    }

    fn get_memory_address(&self) -> u16 {
        self.get_register_pair(&RegisterPair::HL)
    }

    pub fn get_register(&self, register: &Register) -> u8 {
        match register {
            Register::A => self.registers.a,
            Register::B => self.registers.b,
            Register::C => self.registers.c,
            Register::D => self.registers.d,
            Register::E => self.registers.e,
            Register::H => self.registers.h,
            Register::L => self.registers.l,
            Register::Memory => {
                let address = self.get_memory_address();
                self.memory.get(address)
            }
        }
    }

    pub fn set_register(&mut self, register: &Register, value: u8) {
        match register {
            Register::A => self.registers.a = value,
            Register::B => self.registers.b = value,
            Register::C => self.registers.c = value,
            Register::D => self.registers.d = value,
            Register::E => self.registers.e = value,
            Register::H => self.registers.h = value,
            Register::L => self.registers.l = value,
            Register::Memory => {
                let address = self.get_memory_address();
                self.memory.set(address, value);
            }
        }
    }

    pub fn get_register_pair(&self, register_pair: &RegisterPair) -> u16 {
        match register_pair {
            RegisterPair::BC => u16::from_be_bytes([self.registers.b, self.registers.c]),
            RegisterPair::DE => u16::from_be_bytes([self.registers.d, self.registers.e]),
            RegisterPair::HL => u16::from_be_bytes([self.registers.h, self.registers.l]),
            RegisterPair::SP => self.registers.stack_pointer,
        }
    }

    pub fn set_register_pair(&mut self, register_pair: &RegisterPair, value: u16) {
        let [high_byte, low_byte] = value.to_be_bytes();

        match register_pair {
            RegisterPair::BC => {
                self.registers.b = high_byte;
                self.registers.c = low_byte;
            }
            RegisterPair::DE => {
                self.registers.d = high_byte;
                self.registers.e = low_byte;
            }
            RegisterPair::HL => {
                self.registers.h = high_byte;
                self.registers.l = low_byte;
            }
            RegisterPair::SP => self.registers.stack_pointer = value,
        }
    }

    pub fn get_psw(&self) -> u16 {
        return u16::from_be_bytes([self.registers.a, self.condition_flags.get_byte()]);
    }

    pub fn set_psw(&mut self, psw: u16) {
        let [accum_value, flag_byte] = psw.to_be_bytes();

        self.registers.a = accum_value;
        self.condition_flags.set_from_byte(flag_byte);
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
