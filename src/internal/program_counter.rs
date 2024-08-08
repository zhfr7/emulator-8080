use crate::internal::{
    instructions::{Condition, Instruction as I, Register, RegisterPair},
    memory::AddressableMemory,
};

use super::memory::internal::InternalMemory;

#[derive(Debug)]
pub struct ProgramCounter(u16);

impl ProgramCounter {
    pub fn new() -> Self {
        ProgramCounter(0)
    }

    fn increment(&mut self) {
        self.0 = self.0.wrapping_add(1);
    }

    fn get_next_byte(&mut self, memory: &InternalMemory) -> u8 {
        let value = memory.get(self.0);
        self.increment();
        value
    }

    fn get_next_word(&mut self, memory: &InternalMemory) -> u16 {
        let low_byte = self.get_next_byte(memory);
        let high_byte = self.get_next_byte(memory);
        u16::from_be_bytes([high_byte, low_byte])
    }

    pub fn get(&self) -> u16 {
        self.0
    }

    pub fn set(&mut self, address: u16) {
        self.0 = address
    }

    pub fn get_next_instruction(&mut self, memory: &InternalMemory) -> I {
        let byte = memory.get(self.0);

        self.increment();

        match byte {
            // Machine control instructions
            0xC5 => I::PushRegPair(RegisterPair::BC),
            0xD5 => I::PushRegPair(RegisterPair::DE),
            0xE5 => I::PushRegPair(RegisterPair::HL),
            0xF5 => I::PushPSW,
            0xC1 => I::PopRegPair(RegisterPair::BC),
            0xD1 => I::PopRegPair(RegisterPair::DE),
            0xE1 => I::PopRegPair(RegisterPair::HL),
            0xF1 => I::PopPSW,
            0xE3 => I::ExchangeStackTopWithHL,
            0xF9 => I::MoveHLToSP,
            0xDB => I::Input(self.get_next_byte(memory)),
            0xD3 => I::Output(self.get_next_byte(memory)),
            0xFB => I::EnableInterrupts,
            0xF3 => I::DisableInterrupts,
            0x76 => I::Halt,
            0x00 | 0x10 | 0x20 | 0x30 | 0x08 | 0x18 | 0x28 | 0x38 => I::NoOp,

            // Branch instructions
            0xC3 | 0xCB => I::Jump(self.get_next_word(memory)),
            0xC2 => I::ConditionalJump(Condition::NotZero, self.get_next_word(memory)),
            0xCA => I::ConditionalJump(Condition::Zero, self.get_next_word(memory)),
            0xD2 => I::ConditionalJump(Condition::NoCarry, self.get_next_word(memory)),
            0xDA => I::ConditionalJump(Condition::Carry, self.get_next_word(memory)),
            0xE2 => I::ConditionalJump(Condition::OddParity, self.get_next_word(memory)),
            0xEA => I::ConditionalJump(Condition::EvenParity, self.get_next_word(memory)),
            0xF2 => I::ConditionalJump(Condition::Plus, self.get_next_word(memory)),
            0xFA => I::ConditionalJump(Condition::Minus, self.get_next_word(memory)),
            0xCD | 0xDD | 0xED | 0xFD => I::Call(self.get_next_word(memory)),
            0xC4 => I::ConditionalCall(Condition::NotZero, self.get_next_word(memory)),
            0xCC => I::ConditionalCall(Condition::Zero, self.get_next_word(memory)),
            0xD4 => I::ConditionalCall(Condition::NoCarry, self.get_next_word(memory)),
            0xDC => I::ConditionalCall(Condition::Carry, self.get_next_word(memory)),
            0xE4 => I::ConditionalCall(Condition::OddParity, self.get_next_word(memory)),
            0xEC => I::ConditionalCall(Condition::EvenParity, self.get_next_word(memory)),
            0xF4 => I::ConditionalCall(Condition::Plus, self.get_next_word(memory)),
            0xFC => I::ConditionalCall(Condition::Minus, self.get_next_word(memory)),
            0xC9 | 0xD9 => I::Return,
            0xC0 => I::ConditionalReturn(Condition::NotZero),
            0xC8 => I::ConditionalReturn(Condition::Zero),
            0xD0 => I::ConditionalReturn(Condition::NoCarry),
            0xD8 => I::ConditionalReturn(Condition::Carry),
            0xE0 => I::ConditionalReturn(Condition::OddParity),
            0xE8 => I::ConditionalReturn(Condition::EvenParity),
            0xF0 => I::ConditionalReturn(Condition::Plus),
            0xF8 => I::ConditionalReturn(Condition::Minus),
            0xC7 => I::Restart(0),
            0xCF => I::Restart(1),
            0xD7 => I::Restart(2),
            0xDF => I::Restart(3),
            0xE7 => I::Restart(4),
            0xEF => I::Restart(5),
            0xF7 => I::Restart(6),
            0xFF => I::Restart(7),
            0xE9 => I::JumpHLIndirect,

            // Data transfer instructions
            0x7F => I::Move(Register::A, Register::A),
            0x78 => I::Move(Register::B, Register::A),
            0x79 => I::Move(Register::C, Register::A),
            0x7A => I::Move(Register::D, Register::A),
            0x7B => I::Move(Register::E, Register::A),
            0x7C => I::Move(Register::H, Register::A),
            0x7D => I::Move(Register::L, Register::A),
            0x7E => I::Move(Register::Memory, Register::A),
            0x47 => I::Move(Register::A, Register::B),
            0x40 => I::Move(Register::B, Register::B),
            0x41 => I::Move(Register::C, Register::B),
            0x42 => I::Move(Register::D, Register::B),
            0x43 => I::Move(Register::E, Register::B),
            0x44 => I::Move(Register::H, Register::B),
            0x45 => I::Move(Register::L, Register::B),
            0x46 => I::Move(Register::Memory, Register::B),
            0x4F => I::Move(Register::A, Register::C),
            0x48 => I::Move(Register::B, Register::C),
            0x49 => I::Move(Register::C, Register::C),
            0x4A => I::Move(Register::D, Register::C),
            0x4B => I::Move(Register::E, Register::C),
            0x4C => I::Move(Register::H, Register::C),
            0x4D => I::Move(Register::L, Register::C),
            0x4E => I::Move(Register::Memory, Register::C),
            0x57 => I::Move(Register::A, Register::D),
            0x50 => I::Move(Register::B, Register::D),
            0x51 => I::Move(Register::C, Register::D),
            0x52 => I::Move(Register::D, Register::D),
            0x53 => I::Move(Register::E, Register::D),
            0x54 => I::Move(Register::H, Register::D),
            0x55 => I::Move(Register::L, Register::D),
            0x56 => I::Move(Register::Memory, Register::D),
            0x5F => I::Move(Register::A, Register::E),
            0x58 => I::Move(Register::B, Register::E),
            0x59 => I::Move(Register::C, Register::E),
            0x5A => I::Move(Register::D, Register::E),
            0x5B => I::Move(Register::E, Register::E),
            0x5C => I::Move(Register::H, Register::E),
            0x5D => I::Move(Register::L, Register::E),
            0x5E => I::Move(Register::Memory, Register::E),
            0x67 => I::Move(Register::A, Register::H),
            0x60 => I::Move(Register::B, Register::H),
            0x61 => I::Move(Register::C, Register::H),
            0x62 => I::Move(Register::D, Register::H),
            0x63 => I::Move(Register::E, Register::H),
            0x64 => I::Move(Register::H, Register::H),
            0x65 => I::Move(Register::L, Register::H),
            0x66 => I::Move(Register::Memory, Register::H),
            0x6F => I::Move(Register::A, Register::L),
            0x68 => I::Move(Register::B, Register::L),
            0x69 => I::Move(Register::C, Register::L),
            0x6A => I::Move(Register::D, Register::L),
            0x6B => I::Move(Register::E, Register::L),
            0x6C => I::Move(Register::H, Register::L),
            0x6D => I::Move(Register::L, Register::L),
            0x6E => I::Move(Register::Memory, Register::L),
            0x77 => I::Move(Register::A, Register::Memory),
            0x70 => I::Move(Register::B, Register::Memory),
            0x71 => I::Move(Register::C, Register::Memory),
            0x72 => I::Move(Register::D, Register::Memory),
            0x73 => I::Move(Register::E, Register::Memory),
            0x74 => I::Move(Register::H, Register::Memory),
            0x75 => I::Move(Register::L, Register::Memory),
            0x3E => I::MoveImmediate(Register::A, self.get_next_byte(memory)),
            0x06 => I::MoveImmediate(Register::B, self.get_next_byte(memory)),
            0x0E => I::MoveImmediate(Register::C, self.get_next_byte(memory)),
            0x16 => I::MoveImmediate(Register::D, self.get_next_byte(memory)),
            0x1E => I::MoveImmediate(Register::E, self.get_next_byte(memory)),
            0x26 => I::MoveImmediate(Register::H, self.get_next_byte(memory)),
            0x2E => I::MoveImmediate(Register::L, self.get_next_byte(memory)),
            0x36 => I::MoveImmediate(Register::Memory, self.get_next_byte(memory)),
            0x01 => I::LoadRegisterPairImmediate(RegisterPair::BC, self.get_next_word(memory)),
            0x11 => I::LoadRegisterPairImmediate(RegisterPair::DE, self.get_next_word(memory)),
            0x21 => I::LoadRegisterPairImmediate(RegisterPair::HL, self.get_next_word(memory)),
            0x31 => I::LoadRegisterPairImmediate(RegisterPair::SP, self.get_next_word(memory)),
            0x3A => I::LoadAccumDirect(self.get_next_word(memory)),
            0x32 => I::StoreAccumDirect(self.get_next_word(memory)),
            0x2A => I::LoadHLDirect(self.get_next_word(memory)),
            0x22 => I::StoreHLDirect(self.get_next_word(memory)),
            0x0A => I::LoadAccumIndirect(RegisterPair::BC),
            0x1A => I::LoadAccumIndirect(RegisterPair::DE),
            0x02 => I::StoreAccumIndirect(RegisterPair::BC),
            0x12 => I::StoreAccumIndirect(RegisterPair::DE),
            0xEB => I::ExchangeHLWithDE,

            // Arithmetic
            0x87 => I::Add(Register::A),
            0x80 => I::Add(Register::B),
            0x81 => I::Add(Register::C),
            0x82 => I::Add(Register::D),
            0x83 => I::Add(Register::E),
            0x84 => I::Add(Register::H),
            0x85 => I::Add(Register::L),
            0x86 => I::Add(Register::Memory),
            0xC6 => I::AddImmediate(self.get_next_byte(memory)),
            0x8F => I::AddWithCarry(Register::A),
            0x88 => I::AddWithCarry(Register::B),
            0x89 => I::AddWithCarry(Register::C),
            0x8A => I::AddWithCarry(Register::D),
            0x8B => I::AddWithCarry(Register::E),
            0x8C => I::AddWithCarry(Register::H),
            0x8D => I::AddWithCarry(Register::L),
            0x8E => I::AddWithCarry(Register::Memory),
            0xCE => I::AddImmediateWithCarry(self.get_next_byte(memory)),
            0x97 => I::Subtract(Register::A),
            0x90 => I::Subtract(Register::B),
            0x91 => I::Subtract(Register::C),
            0x92 => I::Subtract(Register::D),
            0x93 => I::Subtract(Register::E),
            0x94 => I::Subtract(Register::H),
            0x95 => I::Subtract(Register::L),
            0x96 => I::Subtract(Register::Memory),
            0xD6 => I::SubtractImmediate(self.get_next_byte(memory)),
            0x9F => I::SubtractWithBorrow(Register::A),
            0x98 => I::SubtractWithBorrow(Register::B),
            0x99 => I::SubtractWithBorrow(Register::C),
            0x9A => I::SubtractWithBorrow(Register::D),
            0x9B => I::SubtractWithBorrow(Register::E),
            0x9C => I::SubtractWithBorrow(Register::H),
            0x9D => I::SubtractWithBorrow(Register::L),
            0x9E => I::SubtractWithBorrow(Register::Memory),
            0xDE => I::SubtractImmediateWithBorrow(self.get_next_byte(memory)),
            0x3C => I::Increment(Register::A),
            0x04 => I::Increment(Register::B),
            0x0C => I::Increment(Register::C),
            0x14 => I::Increment(Register::D),
            0x1C => I::Increment(Register::E),
            0x24 => I::Increment(Register::H),
            0x2C => I::Increment(Register::L),
            0x34 => I::Increment(Register::Memory),
            0x3D => I::Decrement(Register::A),
            0x05 => I::Decrement(Register::B),
            0x0D => I::Decrement(Register::C),
            0x15 => I::Decrement(Register::D),
            0x1D => I::Decrement(Register::E),
            0x25 => I::Decrement(Register::H),
            0x2D => I::Decrement(Register::L),
            0x35 => I::Decrement(Register::Memory),
            0x03 => I::IncrementRegPair(RegisterPair::BC),
            0x13 => I::IncrementRegPair(RegisterPair::DE),
            0x23 => I::IncrementRegPair(RegisterPair::HL),
            0x33 => I::IncrementRegPair(RegisterPair::SP),
            0x0B => I::DecrementRegPair(RegisterPair::BC),
            0x1B => I::DecrementRegPair(RegisterPair::DE),
            0x2B => I::DecrementRegPair(RegisterPair::HL),
            0x3B => I::DecrementRegPair(RegisterPair::SP),
            0x09 => I::AddRegPairToHL(RegisterPair::BC),
            0x19 => I::AddRegPairToHL(RegisterPair::DE),
            0x29 => I::AddRegPairToHL(RegisterPair::HL),
            0x39 => I::AddRegPairToHL(RegisterPair::SP),
            0x27 => I::DecimalAdjustAccum,

            // Logical
            0xA7 => I::And(Register::A),
            0xA0 => I::And(Register::B),
            0xA1 => I::And(Register::C),
            0xA2 => I::And(Register::D),
            0xA3 => I::And(Register::E),
            0xA4 => I::And(Register::H),
            0xA5 => I::And(Register::L),
            0xA6 => I::And(Register::Memory),
            0xE6 => I::AndImmediate(self.get_next_byte(memory)),
            0xAF => I::Xor(Register::A),
            0xA8 => I::Xor(Register::B),
            0xA9 => I::Xor(Register::C),
            0xAA => I::Xor(Register::D),
            0xAB => I::Xor(Register::E),
            0xAC => I::Xor(Register::H),
            0xAD => I::Xor(Register::L),
            0xAE => I::Xor(Register::Memory),
            0xEE => I::XorImmediate(self.get_next_byte(memory)),
            0xB7 => I::Or(Register::A),
            0xB0 => I::Or(Register::B),
            0xB1 => I::Or(Register::C),
            0xB2 => I::Or(Register::D),
            0xB3 => I::Or(Register::E),
            0xB4 => I::Or(Register::H),
            0xB5 => I::Or(Register::L),
            0xB6 => I::Or(Register::Memory),
            0xF6 => I::OrImmediate(self.get_next_byte(memory)),
            0xBF => I::Compare(Register::A),
            0xB8 => I::Compare(Register::B),
            0xB9 => I::Compare(Register::C),
            0xBA => I::Compare(Register::D),
            0xBB => I::Compare(Register::E),
            0xBC => I::Compare(Register::H),
            0xBD => I::Compare(Register::L),
            0xBE => I::Compare(Register::Memory),
            0xFE => I::CompareImmediate(self.get_next_byte(memory)),
            0x07 => I::RotateLeft,
            0x0F => I::RotateRight,
            0x17 => I::RotateLeftThroughCarry,
            0x1F => I::RotateRightThroughCarry,
            0x2F => I::ComplementAccum,
            0x3F => I::ComplementCarry,
            0x37 => I::SetCarry,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_all_instructions() {
        let mut memory = InternalMemory::new();
        let program: Vec<u8> = vec![
            0x00, 0x01, 0x55, 0x44, 0x02, 0x03, 0x04, 0x05, 0x06, 0x99, 0x07, 0x08, 0x09, 0x0A,
            0x0B, 0x0C, 0x0D, 0x0E, 0x98, 0x0F, 0x10, 0x11, 0x77, 0x66, 0x12, 0x13, 0x14, 0x15,
            0x16, 0x88, 0x17, 0x18,
        ];

        let program_length = program.len();

        memory.set_range(0x0000, program_length as u16, program);

        let mut program_counter = ProgramCounter::new();
        let mut instructions: Vec<I> = vec![];

        while (program_counter.0 as usize) < program_length {
            instructions.push(program_counter.get_next_instruction(&memory));
        }

        assert_eq!(
            instructions,
            vec![
                I::NoOp,
                I::LoadRegisterPairImmediate(RegisterPair::BC, 0x4455),
                I::StoreAccumIndirect(RegisterPair::BC),
                I::IncrementRegPair(RegisterPair::BC),
                I::Increment(Register::B),
                I::Decrement(Register::B),
                I::MoveImmediate(Register::B, 0x99),
                I::RotateLeft,
                I::NoOp,
                I::AddRegPairToHL(RegisterPair::BC),
                I::LoadAccumIndirect(RegisterPair::BC),
                I::DecrementRegPair(RegisterPair::BC),
                I::Increment(Register::C),
                I::Decrement(Register::C),
                I::MoveImmediate(Register::C, 0x98),
                I::RotateRight,
                I::NoOp,
                I::LoadRegisterPairImmediate(RegisterPair::DE, 0x6677),
                I::StoreAccumIndirect(RegisterPair::DE),
                I::IncrementRegPair(RegisterPair::DE),
                I::Increment(Register::D),
                I::Decrement(Register::D),
                I::MoveImmediate(Register::D, 0x88),
                I::RotateLeftThroughCarry,
                I::NoOp
            ]
        )
    }
}
