use std::{hash::Hash, sync::LazyLock};

use hashbrown::HashSet;

pub static CPU_OP_CODES: LazyLock<HashSet<OpCode>> = LazyLock::new(|| {
    use AddressingMode::*;
    use Instruction::*;
    let contents = &[
        // ADC
        OpCode::new(0x69, ADC, 2, 2, Immediate),
        OpCode::new(0x65, ADC, 2, 3, ZeroPage),
        OpCode::new(0x75, ADC, 2, 4, ZeroPageX),
        OpCode::new(0x6D, ADC, 3, 4, Absolute),
        OpCode::new(0x7D, ADC, 3, 4, AbsoluteX),
        OpCode::new(0x79, ADC, 3, 4, AbsoluteY),
        OpCode::new(0x61, ADC, 2, 6, IndirectX),
        OpCode::new(0x71, ADC, 2, 5, IndirectY),
        // AND
        OpCode::new(0x29, AND, 2, 2, Immediate),
        OpCode::new(0x25, AND, 2, 3, ZeroPage),
        OpCode::new(0x35, AND, 2, 4, ZeroPageX),
        OpCode::new(0x2D, AND, 3, 4, Absolute),
        OpCode::new(0x3D, AND, 3, 4, AbsoluteX),
        OpCode::new(0x39, AND, 3, 4, AbsoluteY),
        OpCode::new(0x21, AND, 2, 6, IndirectX),
        OpCode::new(0x31, AND, 2, 5, IndirectY),
        // ASL
        OpCode::new(0x0A, ASL, 1, 2, Other),
        OpCode::new(0x06, ASL, 2, 5, ZeroPage),
        OpCode::new(0x16, ASL, 2, 6, ZeroPageX),
        OpCode::new(0x0E, ASL, 3, 6, Absolute),
        OpCode::new(0x1E, ASL, 3, 7, AbsoluteX),
        // BCC
        OpCode::new(0x90, BCC, 2, 2, Other),
        // BCS
        OpCode::new(0xB0, BCS, 2, 2, Other),
        // BEQ
        OpCode::new(0xF0, BEQ, 2, 2, Other),
        // BIT
        OpCode::new(0x24, BIT, 2, 3, ZeroPage),
        OpCode::new(0x2C, BIT, 3, 4, Absolute),
        // BMI
        OpCode::new(0x30, BMI, 2, 2, Other),
        // BNE
        OpCode::new(0xD0, BNE, 2, 2, Other),
        // BPL
        OpCode::new(0x10, BPL, 2, 2, Other),
        // BRK
        OpCode::new(0x00, BRK, 0, 7, Other),
        // BVC
        OpCode::new(0x50, BVC, 2, 2, Other),
        // BVS
        OpCode::new(0x70, BVS, 2, 2, Other),
        // CLC
        OpCode::new(0x18, CLC, 1, 2, Other),
        // CLD #[NOTE] Not used in NES emulation
        // CLI
        OpCode::new(0x58, CLI, 1, 2, Other),
        // CLV
        OpCode::new(0xB8, CLV, 1, 2, Other),
        // CMP
        OpCode::new(0xC9, CMP, 2, 2, Immediate),
        OpCode::new(0xC5, CMP, 2, 3, ZeroPage),
        OpCode::new(0xD5, CMP, 2, 4, ZeroPageX),
        OpCode::new(0xCD, CMP, 3, 4, Absolute),
        OpCode::new(0xDD, CMP, 3, 4, AbsoluteX),
        OpCode::new(0xD9, CMP, 3, 4, AbsoluteY),
        OpCode::new(0xC1, CMP, 2, 6, IndirectX),
        OpCode::new(0xD1, CMP, 2, 5, IndirectY),
        // CPX
        OpCode::new(0xE0, CPX, 2, 2, Immediate),
        OpCode::new(0xE4, CPX, 2, 3, ZeroPage),
        OpCode::new(0xEC, CPX, 3, 4, Absolute),
        // CPY
        OpCode::new(0xC0, CPY, 2, 2, Immediate),
        OpCode::new(0xC4, CPY, 2, 3, ZeroPage),
        OpCode::new(0xCC, CPY, 3, 4, Absolute),
        // DEC
        OpCode::new(0xC6, DEC, 2, 5, ZeroPage),
        OpCode::new(0xD6, DEC, 2, 6, ZeroPageX),
        OpCode::new(0xCE, DEC, 3, 6, Absolute),
        OpCode::new(0xDE, DEC, 4, 7, AbsoluteX),
        // DEX
        OpCode::new(0xCA, DEX, 1, 2, Other),
        // DEY
        OpCode::new(0x88, DEY, 1, 2, Other),
        // EOR
        OpCode::new(0x49, EOR, 2, 2, Immediate),
        OpCode::new(0x45, EOR, 2, 3, ZeroPage),
        OpCode::new(0x55, EOR, 2, 4, ZeroPageX),
        OpCode::new(0x4D, EOR, 3, 4, Absolute),
        OpCode::new(0x5D, EOR, 3, 4, AbsoluteX),
        OpCode::new(0x59, EOR, 3, 4, AbsoluteY),
        OpCode::new(0x41, EOR, 2, 6, IndirectX),
        OpCode::new(0x51, EOR, 2, 5, IndirectY),
        // INC
        OpCode::new(0xE6, INC, 2, 5, ZeroPage),
        OpCode::new(0xF6, INC, 2, 6, ZeroPageX),
        OpCode::new(0xEE, INC, 3, 6, Absolute),
        OpCode::new(0xFE, INC, 3, 5, AbsoluteX),
        // INX
        OpCode::new(0xE8, INX, 1, 2, Other),
        // INY
        OpCode::new(0xC8, INY, 1, 2, Other),
        // JMP
        OpCode::new(0x4C, JMP, 3, 3, Absolute),
        OpCode::new(0x6C, JMP, 3, 5, Other),
        // JSR
        OpCode::new(0x20, JSR, 3, 6, Absolute),
        // LDA
        OpCode::new(0xA9, LDA, 2, 2, Immediate),
        OpCode::new(0xA5, LDA, 2, 3, ZeroPage),
        OpCode::new(0xB5, LDA, 2, 4, ZeroPageX),
        OpCode::new(0xAD, LDA, 3, 4, Absolute),
        OpCode::new(0xBD, LDA, 3, 4, AbsoluteX),
        OpCode::new(0xB9, LDA, 3, 4, AbsoluteY),
        OpCode::new(0xA1, LDA, 2, 6, IndirectX),
        OpCode::new(0xB1, LDA, 2, 5, IndirectY),
        // LDX
        OpCode::new(0xA2, LDX, 2, 2, Immediate),
        OpCode::new(0xA6, LDX, 2, 3, ZeroPage),
        OpCode::new(0xB2, LDX, 2, 4, ZeroPageY),
        OpCode::new(0xAE, LDX, 3, 4, Absolute),
        OpCode::new(0xBE, LDX, 3, 4, AbsoluteY),
        // LDY
        OpCode::new(0xA0, LDY, 2, 2, Immediate),
        OpCode::new(0xA4, LDY, 2, 3, ZeroPage),
        OpCode::new(0xB4, LDY, 2, 4, ZeroPageX),
        OpCode::new(0xAC, LDY, 3, 4, Absolute),
        OpCode::new(0xBC, LDY, 3, 4, AbsoluteX),
        // LSR
        OpCode::new(0x4A, LSR, 1, 2, Other),
        OpCode::new(0x46, LSR, 2, 5, ZeroPage),
        OpCode::new(0x56, LSR, 2, 6, ZeroPageY),
        OpCode::new(0x4E, LSR, 3, 6, Absolute),
        OpCode::new(0x5E, LSR, 3, 7, AbsoluteX),
        // NOP
        OpCode::new(0xEA, NOP, 1, 2, Other),
        // ORA
        OpCode::new(0x09, ORA, 2, 2, Immediate),
        OpCode::new(0x05, ORA, 2, 3, ZeroPage),
        OpCode::new(0x15, ORA, 2, 4, ZeroPageX),
        OpCode::new(0x0D, ORA, 3, 4, Absolute),
        OpCode::new(0x1D, ORA, 3, 4, AbsoluteX),
        OpCode::new(0x19, ORA, 3, 4, AbsoluteY),
        OpCode::new(0x01, ORA, 2, 6, IndirectX),
        OpCode::new(0x11, ORA, 2, 5, IndirectY),
        // PHA
        OpCode::new(0x48, PHA, 1, 3, Other),
        // PHP
        OpCode::new(0x08, PHP, 1, 3, Other),
        // PLA
        OpCode::new(0x68, PLA, 1, 4, Other),
        // PLP
        OpCode::new(0x28, PLP, 1, 4, Other),
        // ROL
        OpCode::new(0x2A, ROL, 1, 2, Other),
        OpCode::new(0x26, ROL, 2, 5, ZeroPage),
        OpCode::new(0x36, ROL, 2, 6, ZeroPageX),
        OpCode::new(0x2E, ROL, 3, 6, Absolute),
        OpCode::new(0x3E, ROL, 3, 7, AbsoluteY),
        // ROR
        OpCode::new(0x6A, ROL, 1, 2, Other),
        OpCode::new(0x66, ROL, 2, 5, ZeroPage),
        OpCode::new(0x76, ROL, 2, 6, ZeroPageX),
        OpCode::new(0x6E, ROL, 3, 6, Absolute),
        OpCode::new(0x7E, ROL, 3, 7, AbsoluteY),
        // RTI
        OpCode::new(0x40, RTI, 1, 6, Other),
        // RTS
        OpCode::new(0x60, RTS, 1, 6, Other),
        // SBC
        OpCode::new(0xE9, SBC, 2, 2, Immediate),
        OpCode::new(0xE5, SBC, 2, 3, ZeroPage),
        OpCode::new(0xF5, SBC, 2, 4, ZeroPageX),
        OpCode::new(0xED, SBC, 3, 4, Absolute),
        OpCode::new(0xFD, SBC, 3, 4, AbsoluteX),
        OpCode::new(0xF9, SBC, 3, 4, AbsoluteY),
        OpCode::new(0xE1, SBC, 2, 6, IndirectX),
        OpCode::new(0xF1, SBC, 2, 5, IndirectY),
        // SEC
        OpCode::new(0x38, SEC, 1, 2, Other),
        // SED [NOTE] Decimal mode not used in NES chip
        // SEI
        OpCode::new(0x78, SEI, 1, 2, Other),
        // STA
        OpCode::new(0x85, STA, 2, 3, ZeroPage),
        OpCode::new(0x95, STA, 2, 4, ZeroPageX),
        OpCode::new(0x8D, STA, 3, 4, Absolute),
        OpCode::new(0x9D, STA, 3, 5, AbsoluteX),
        OpCode::new(0x99, STA, 3, 5, AbsoluteY),
        OpCode::new(0x81, STA, 2, 6, IndirectX),
        OpCode::new(0x91, STA, 2, 6, IndirectY),
        // STX
        OpCode::new(0x86, STX, 2, 3, ZeroPage),
        OpCode::new(0x96, STX, 2, 4, ZeroPageY),
        OpCode::new(0x8E, STX, 3, 4, Absolute),
        // STY
        OpCode::new(0x84, STY, 2, 3, ZeroPage),
        OpCode::new(0x94, STY, 2, 4, ZeroPageX),
        OpCode::new(0x8C, STY, 3, 4, Absolute),
        // TAX
        OpCode::new(0xAA, TAX, 1, 2, Other),
        // TAY
        OpCode::new(0xA8, TAY, 1, 2, Other),
        // TSX
        OpCode::new(0xBA, TSX, 1, 2, Other),
        // TXA
        OpCode::new(0x8A, TXA, 1, 2, Other),
        // TXS
        OpCode::new(0x9A, TXS, 1, 2, Other),
        // TYA
        OpCode::new(0x98, TYA, 1, 2, Other),
    ];
    HashSet::from_iter(contents.iter().cloned())
});

#[derive(Debug, Clone)]
pub struct OpCode {
    code: u8,
    pub instruction: Instruction,
    pub len: u8,
    #[allow(dead_code)]
    cycles: u8,
    pub addressing_mode: AddressingMode,
}

impl Eq for OpCode {}

impl PartialEq for OpCode {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code
    }
}

impl hashbrown::Equivalent<OpCode> for u8 {
    fn equivalent(&self, key: &OpCode) -> bool {
        self == &key.code
    }
}

impl PartialEq<u8> for OpCode {
    fn eq(&self, other: &u8) -> bool {
        self.code == *other
    }
}

impl Hash for OpCode {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.code.hash(state);
    }
}

impl OpCode {
    pub fn new(
        code: u8,
        instruction: Instruction,
        bytes: u8,
        cycles: u8,
        addressing_mode: AddressingMode,
    ) -> Self {
        Self {
            code,
            instruction,
            len: bytes,
            cycles,
            addressing_mode,
        }
    }
}
#[derive(Debug, Clone)]
pub enum Instruction {
    ADC,
    AND,
    ASL,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    JMP,
    JSR,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    ROL,
    ROR,
    RTI,
    RTS,
    SBC,
    SEC,
    SEI,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
}

#[derive(Debug, Clone)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
    /// i.e. Implied, Relative or Accumulator
    Other,
}
