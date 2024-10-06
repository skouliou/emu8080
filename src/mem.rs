use std::{
    error::Error,
    fmt::Display,
    fs::File,
    io::{BufReader, Read},
    os::unix::fs::MetadataExt,
};

#[derive(Default)]
pub(crate) struct Memory {
    mem: Box<[u8]>,
}

impl Memory {
    pub(crate) fn new() -> Self {
        todo!()
    }

    /// open file load it into memory
    pub(crate) fn load_file(path: &str) -> Result<Memory, Box<dyn Error>> {
        let file = File::open(path)?;
        let file_size = file.metadata()?.size() as usize;

        // file is larger than what memory can handle
        if file_size > 0xFFFF {
            return Err(Box::new(NotEnoughMemory(file_size)));
        }

        let mut file = BufReader::new(file);
        let mut buffer = Vec::with_capacity(file_size);

        file.read_to_end(&mut buffer)?;

        let mem = Memory {
            mem: Box::from(buffer),
        };
        Ok(mem)
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

#[derive(Debug)]
pub(crate) struct NotEnoughMemory(usize);

impl Display for NotEnoughMemory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "NotEnoughMemory: file size ({})is larger than memory size",
            self.0
        )
    }
}
impl Error for NotEnoughMemory {}

pub(crate) struct InvalidAddress;
