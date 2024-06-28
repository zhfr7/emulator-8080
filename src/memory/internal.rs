use crate::memory::Address;

use super::AddressableMemory;

#[derive(Debug)]
pub struct InternalMemory([u8; u16::MAX as usize]);

impl InternalMemory {
    pub fn new() -> Self {
        InternalMemory([0; u16::MAX as usize])
    }
}

impl AddressableMemory<u16> for InternalMemory {
    fn get_range(&self, start: u16, end: u16) -> Vec<u8> {
        self.0[(start.to_usize())..=(end.to_usize())].to_owned()
    }

    fn set_range(&mut self, start: u16, end: u16, bytes: Vec<u8>) {
        for (byte, address) in bytes.into_iter().zip(start.to_usize()..=end.to_usize()) {
            self.0[address] = byte
        }
    }
}
