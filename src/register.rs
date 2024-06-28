use crate::instructions::{Condition, Register, RegisterPair};

#[derive(Debug, Default)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct ConditionFlags {
    pub sign: bool,
    pub zero: bool,
    pub parity: bool,
    pub carry: bool,
    pub aux_carry: bool,
}

#[derive(Debug, Default)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub condition_flags: ConditionFlags,
    pub stack_pointer: u16,
}

impl Registers {
    pub fn get(&self, register: &Register) -> u8 {
        match register {
            Register::A => self.a,
            Register::B => self.b,
            Register::C => self.c,
            Register::D => self.d,
            Register::E => self.e,
            Register::H => self.h,
            Register::L => self.l,
        }
    }

    pub fn set(&mut self, register: &Register, value: u8) {
        match register {
            Register::A => self.a = value,
            Register::B => self.b = value,
            Register::C => self.c = value,
            Register::D => self.d = value,
            Register::E => self.e = value,
            Register::H => self.h = value,
            Register::L => self.l = value,
        }
    }

    pub fn get_pair(&self, register_pair: &RegisterPair) -> u16 {
        match register_pair {
            RegisterPair::BC => u16::from_be_bytes([self.b, self.c]),
            RegisterPair::DE => u16::from_be_bytes([self.d, self.e]),
            RegisterPair::HL => u16::from_be_bytes([self.h, self.l]),
            RegisterPair::SP => self.stack_pointer,
        }
    }

    pub fn set_pair(&mut self, register_pair: &RegisterPair, value: u16) {
        let [high_byte, low_byte] = value.to_be_bytes();

        match register_pair {
            RegisterPair::BC => {
                self.b = high_byte;
                self.c = low_byte;
            }
            RegisterPair::DE => {
                self.d = high_byte;
                self.e = low_byte;
            }
            RegisterPair::HL => {
                self.h = high_byte;
                self.l = low_byte;
            }
            RegisterPair::SP => self.stack_pointer = value,
        }
    }

    pub fn get_memory_address(&self) -> u16 {
        self.get_pair(&RegisterPair::HL)
    }

    pub fn set_zero_sign_parity_flags(&mut self, value: u8) {
        self.condition_flags.zero = value == 0;
        self.condition_flags.sign = value >> 7 == 0;
        self.condition_flags.parity = value.count_ones() % 2 == 0;
    }

    pub fn set_carry_flag(&mut self, value: bool) {
        self.condition_flags.carry = value
    }

    pub fn set_aux_carry_flag(&mut self, value: bool) {
        self.condition_flags.aux_carry = value
    }

    pub fn get_psw(&self) -> u16 {
        let carry_bit: u8 = if self.condition_flags.carry { 1 } else { 0 };
        let parity_bit: u8 = if self.condition_flags.parity {
            1 << 2
        } else {
            0
        };
        let aux_carry_bit: u8 = if self.condition_flags.aux_carry {
            1 << 4
        } else {
            0
        };
        let zero_bit: u8 = if self.condition_flags.zero { 1 << 6 } else { 0 };
        let sign_bit: u8 = if self.condition_flags.sign { 1 << 7 } else { 0 };

        let low_byte = 0b00000010 | carry_bit | parity_bit | aux_carry_bit | zero_bit | sign_bit;
        let high_byte = self.a;

        u16::from_be_bytes([high_byte, low_byte])
    }

    pub fn set_psw(&mut self, psw: u16) {
        let [high_byte, low_byte] = psw.to_be_bytes();

        let carry = low_byte & 1 > 0;
        let parity = low_byte & (1 << 2) > 0;
        let aux_carry = low_byte & (1 << 4) > 0;
        let zero = low_byte & (1 << 6) > 0;
        let sign = low_byte & (1 << 7) > 0;

        self.condition_flags.carry = carry;
        self.condition_flags.parity = parity;
        self.condition_flags.aux_carry = aux_carry;
        self.condition_flags.zero = zero;
        self.condition_flags.sign = sign;

        self.a = high_byte;
    }

    pub fn is_condition_fulfilled(&self, condition: &Condition) -> bool {
        match condition {
            Condition::NotZero => !self.condition_flags.zero,
            Condition::Zero => self.condition_flags.zero,
            Condition::NoCarry => !self.condition_flags.carry,
            Condition::Carry => self.condition_flags.carry,
            Condition::OddParity => !self.condition_flags.carry,
            Condition::EvenParity => self.condition_flags.carry,
            Condition::Plus => !self.condition_flags.sign,
            Condition::Minus => self.condition_flags.sign,
        }
    }
}
