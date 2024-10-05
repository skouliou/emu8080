use crate::mem::Memory;
use crate::cpu::CPU;

pub(crate) struct Instruction {
    /// Instruction Opcode
    opcode: Opcode,
    /// Execution duration (when not executed / when executed)
    duration: (u8 /* not executed */, u8 /* executed */),
    /// Instruction side effects
    effect: Box<dyn FnOnce(&mut CPU, &mut Memory) -> ()>,
    /// Memory size in bytes
    size: u8,
}

#[derive(Debug)]
enum Opcode {
    // carry bit instructions
    /// Complement carry flag
    CMC, // complement carry
    /// Set Carry Flag
    STC, // set carry

    // single register instructions
    /// Increment instruction
    INR,
    /// Decrement instruction
    DCR,
    /// Complement Accumulator
    CMA,
    /// Decimal Adjust Accumulator
    DAA,

    /// No Operation
    NOP,

    // data transfer instructions
    /// Move Instruction
    MOV,
    /// Store Accumulator
    STAX,
    /// Load Accumulator
    LDAX,

    // register or memory to accumulator instructions
    /// Add Register or Memory to Accumulator
    ADD,
    /// Add Register or Memory to Accumulator with Carry
    ADC,
    /// Subtract Register or Memory to Accumulator
    SUB,
    /// Subtract register or memory to accumulator with borrow
    SBB,
    /// Logical AND register or memory with accumulator
    ANL,
    /// Logical XOR register or memory with accumulator
    XRA,
    /// Logical OR register or memory with accumulator
    ORA,
    /// Compare register or memory with accumulator
    CMP,

    // rotate accumulator instructions
    /// Rotate Accumulator Left
    RLC,
    /// Rotate Accumulator Right
    RRC,
    /// Rotate Accumulator Left Through Carry
    RAL,
    /// Rotate Accumulator Right Through Carry
    RAR,

    // register pair instructions
    /// Push data into stack
    PUSH,
    /// Pop data off stack
    POP,
    /// Double add
    DAD,
    /// Increment register pair
    INX,
    /// Decrement register pair
    DCX,
    /// Exchange registers
    XCHG,
    /// Exchange stack
    XCHL,
    /// Load registers (H:L) to stack pointer (SP)
    SPHL,

    // immediate instructions
    /// Move immediate data into register
    MVI,
    /// Add immediate data into register
    ADI,
    /// Add immediate data into register with carry
    ACI,
    /// Subtract immediate data into register
    SUI,
    /// Subtract immediate data into register with borrow
    SBI,
    /// AND immediate data with accumulator
    ANI,
    /// XOR immediate data with accumulator
    XRI,
    /// OR immediate data with accumulator
    ORI,
    /// Compare immediate data with accumulator
    CPI,
    // direct addressing instructions
    /// Store accumulator direct
    STA,
    /// Load accumulator direct
    LDA,
    /// Store H:L registers direct
    SHLD,
    /// Load H:L registers direct
    LHLD,

    // jump instructions
    /// Load program counter
    PCHL,
    /// Unconditional jump
    JMP,
    /// Jump if carry flag is set
    JC,
    /// Jump if carry flag is not set
    JNC,
    /// Jump if zero flag is set
    JZ,
    /// Jump if zero flag is not set
    JNZ,
    /// Jump if sign flag is set
    JM,
    /// Jump if sign flag is not set
    JP,
    /// Jump if parity flag is set
    JPE,
    /// Jump if parity flag is not set
    JPO,
    // call subroutine instructions
    /// A call operation is unconditionally performed
    CALL,
    /// If the carry bit is set, a call operation is performed
    CC,
    /// If the carry bit is not set, a call operation is performed
    CNC,
    /// If the zero bit is set, a call operation is performed
    CZ,
    /// If the zero bit is not set, a call operation is performed
    CNZ,
    /// If the sign bit is set, a call operation is performed
    CM,
    /// If the sign bit is not set, a call operation is performed
    CP,
    /// If the parity bit is set, a call operation is performed
    CPE,
    /// If the parity bit is not set, a call operation is performed
    CPO,

    /// A return operation is unconditionally performed
    RET,
    /// If the carry bit is set, a return operation is performed
    RC,
    /// If the carry bit is not set, a return operation is performed
    RNC,
    /// If the zero bit is set, a return operation is performed
    RZ,
    /// If the zero bit is not set, a return operation is performed
    RNZ,
    /// If the sign bit is set, a return operation is performed
    RM,
    /// If the sign bit is not set, a return operation is performed
    RP,
    /// If the parity bit is set, a return operation is performed
    RPE,
    /// If the parity bit is not set, a return operation is performed
    RPO,

    /// Restart instruction
    RST,
    // interrupt instructions
    /// Enable interrupts
    EI,
    /// Disable interrupts
    DI,

    // I/O instructions
    /// Input instruction
    IN,
    /// Output instruction
    OUT,

    /// Halt instruction
    HLT,
}
