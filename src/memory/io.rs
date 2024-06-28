use super::{Address, AddressableMemory};

#[derive(Debug)]
pub struct IOMemory([u8; u8::MAX as usize]);

impl IOMemory {
    pub fn new() -> Self {
        IOMemory([0; u8::MAX as usize])
    }
}

impl AddressableMemory<u8> for IOMemory {
    fn get_range(&self, start: u8, end: u8) -> Vec<u8> {
        self.0[(start.to_usize())..=(end.to_usize())].to_owned()
    }

    fn set_range(&mut self, start: u8, end: u8, bytes: Vec<u8>) {
        for (byte, address) in bytes.into_iter().zip(start.to_usize()..=end.to_usize()) {
            self.0[address] = byte
        }
    }
}
