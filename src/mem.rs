pub(crate) struct Memory {
    mem: [u8; 0xFFFF], // 65,536 (2^16) bytes
}

impl Memory {
    pub(crate) fn new() -> Self {
        todo!()
    }

    /// open file in binary mode and load it into memory
    pub(crate) fn load_file(path: &str) -> std::io::Result<Self> {
        todo!()
    }

    /// get the content held in given memory address
    pub(crate) fn get(&self, address: u16) -> Result<u8, InvalidAddress> {
        todo!()
    }

    /// set the memory content of given memory address to data
    pub(crate) fn set(&self, address: u16, data: u8) -> Result<(), InvalidAddress> {
        todo!()
    }
}

pub(crate) struct InvalidAddress;
