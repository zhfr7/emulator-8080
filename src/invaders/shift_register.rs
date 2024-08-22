pub struct ShiftRegister {
    offset: u8,
    state: u16
}

impl ShiftRegister {
    pub fn new() -> Self {
        ShiftRegister { offset: 0, state: 0 }
    }

    pub fn set_offset(&mut self, value: u8) {
        self.offset = value;
    }

    pub fn push_value(&mut self, value: u8) {
        self.state = (value as u16) << 8 | self.state >> 8;
    }

    pub fn get_shifted_value(&mut self) -> u8 {
        ((self.state << self.offset) >> 8) as u8
    }
}
