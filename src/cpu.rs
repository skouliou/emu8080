use std::fmt::Display;

use crate::{asm::Instruction, mem::{InvalidAddress, Memory}};

#[derive(Debug, Default)]
pub(crate) struct CPU {
    regs: Registers,
    sp: u16,         // 16bit stack pointer
    pc: u16,         // 16bit program counter
    flags: CpuState, // ineternal state after last instruction
}

#[derive(Debug, Default)]
struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    l: u8,
    h: u8,
}

#[derive(Debug, Default)]
struct CpuState {
    carry: bool,
    auxcarry: bool,
    sign: bool,
    zero: bool,
    parity: bool,
    interrupt: bool,
    halted: bool,
}

impl CPU {
    pub(crate) fn new(self: &Self) -> Self {
        Self {
            regs: Registers {
                ..Default::default()
            },
            pc: 0,
            sp: 0,
            flags: CpuState {
                ..Default::default()
            },
        }
    }

    pub(crate) fn reset(mut self) {
        self = Default::default();
    }

    fn fetch(&mut self, memory: &Memory) -> Result<u8, InvalidAddress> {
        let val = memory.get(self.pc)?;
        todo!("check instruction size");
        Ok(val)
    }

    fn decode(&self, instruction: Instruction) -> Result<(), DecoderError> {
        todo!()
    }

    fn execute(&self, instruction: Instruction) -> Result<(), ExecutionError> {
        todo!()
    }

    fn run(&self) {
        todo!()
    }
}

struct MemoryError {
    address: u8,
}

impl Display for MemoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MemoryError: address [{:08x}]", self.address)
    }
}

struct DecoderError {
    address: u8,
    instruction: Instruction,
}

impl Display for DecoderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DecoderError: address [{:08x}]", self.address)
    }
}

struct ExecutionError;
