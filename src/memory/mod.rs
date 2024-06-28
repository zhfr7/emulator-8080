pub mod internal;
pub mod io;

pub trait Address {
    fn to_usize(self) -> usize;
}

impl Address for u8 {
    fn to_usize(self) -> usize {
        self as usize
    }
}

impl Address for u16 {
    fn to_usize(self) -> usize {
        self as usize
    }
}

pub trait AddressableMemory<T>
where
    T: Address + Copy,
{
    fn get_range(&self, start: T, end: T) -> Vec<u8>;
    fn set_range(&mut self, start: T, end: T, bytes: Vec<u8>);

    fn get(&self, address: T) -> u8 {
        self.get_range(address, address)[0]
    }
    fn set(&mut self, address: T, value: u8) {
        self.set_range(address, address, vec![value])
    }
}
