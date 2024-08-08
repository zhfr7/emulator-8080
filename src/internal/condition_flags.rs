use super::instructions::Condition;

#[derive(Debug, Default)]
pub struct ConditionFlags {
    pub sign: bool,
    pub zero: bool,
    pub parity: bool,
    pub carry: bool,
    pub aux_carry: bool,
}

fn to_bitflag(value: bool, position: usize) -> u8 {
    (if value { 1 } else { 0 }) << position
}

fn from_bitflag(byte: u8, position: usize) -> bool {
    byte & (1 << position) != 0
}

impl ConditionFlags {
    pub fn set_zero_sign_parity_flags(&mut self, value: u8) {
        self.zero = value == 0;
        self.sign = value >> 7 == 1;
        self.parity = value.count_ones() % 2 == 0;
    }

    pub fn get_byte(&self) -> u8 {
        let carry_bit: u8 = to_bitflag(self.carry, 0);
        let parity_bit: u8 = to_bitflag(self.parity, 2);
        let aux_carry_bit: u8 = to_bitflag(self.aux_carry, 4);
        let zero_bit: u8 = to_bitflag(self.zero, 6);
        let sign_bit: u8 = to_bitflag(self.sign, 7);

        0x02 | carry_bit | parity_bit | aux_carry_bit | zero_bit | sign_bit
    }

    pub fn set_from_byte(&mut self, flag_byte: u8) {
        let carry = from_bitflag(flag_byte, 0);
        let parity = from_bitflag(flag_byte, 2);
        let aux_carry = from_bitflag(flag_byte, 4);
        let zero = from_bitflag(flag_byte, 6);
        let sign = from_bitflag(flag_byte, 7);

        self.carry = carry;
        self.parity = parity;
        self.aux_carry = aux_carry;
        self.zero = zero;
        self.sign = sign;
    }

    pub fn is_condition_fulfilled(&self, condition: &Condition) -> bool {
        match condition {
            Condition::NotZero => !self.zero,
            Condition::Zero => self.zero,
            Condition::NoCarry => !self.carry,
            Condition::Carry => self.carry,
            Condition::OddParity => !self.parity,
            Condition::EvenParity => self.parity,
            Condition::Plus => !self.sign,
            Condition::Minus => self.sign,
        }
    }
}
