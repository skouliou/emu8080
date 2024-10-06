use crate::{
    asm::Instruction,
    mem::{InvalidAddress, Memory},
};
use std::fmt::Display;

#[derive(Debug, Default)]
pub(crate) struct CPU {
    pub(crate) regs: Registers,
    pub(crate) sp: u16,         // 16bit stack pointer
    pub(crate) pc: u16,         // 16bit program counter
    pub(crate) flags: CpuState, // ineternal state after last instruction
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
pub(crate) struct CpuState {
    pub(crate) carry: bool,
    pub(crate) auxcarry: bool,
    pub(crate) sign: bool,
    pub(crate) zero: bool,
    pub(crate) parity: bool,
    pub(crate) interrupt: bool,
    pub(crate) halted: bool,
}

impl CPU {
    pub(crate) fn new(self: &Self) -> Self {
        Default::default()
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

    fn execute(
        &mut self,
        instruction: &mut Instruction,
        mem: &mut Memory,
    ) -> Result<(), ExecutionError> {
        self.pc += instruction.size as u16;
        (instruction.effect)(self, mem)
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
