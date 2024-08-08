use std::fmt::Display;

use super::{Condition, Instruction, Register, RegisterPair};

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Register::A => write!(f, "A"),
            Register::B => write!(f, "B"),
            Register::C => write!(f, "C"),
            Register::D => write!(f, "D"),
            Register::E => write!(f, "E"),
            Register::H => write!(f, "H"),
            Register::L => write!(f, "L"),
            Register::Memory => write!(f, "M"),
        }
    }
}

impl Display for RegisterPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegisterPair::BC => write!(f, "BC"),
            RegisterPair::DE => write!(f, "DE"),
            RegisterPair::HL => write!(f, "HL"),
            RegisterPair::SP => write!(f, "SP"),
        }
    }
}

impl Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Condition::NotZero => write!(f, "NZ"),
            Condition::Zero => write!(f, "Z "),
            Condition::NoCarry => write!(f, "NC"),
            Condition::Carry => write!(f, "C "),
            Condition::OddParity => write!(f, "PO"),
            Condition::EvenParity => write!(f, "PE"),
            Condition::Plus => write!(f, "P "),
            Condition::Minus => write!(f, "M "),
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Instruction::Move(r2, r1) => write!(f, "MOV     {}, {}", r1, r2),
            Instruction::MoveImmediate(r, data) => write!(f, "MVI     {}, {:#04x}", r, data),
            Instruction::LoadRegisterPairImmediate(rp, data) => {
                write!(f, "LXI     {}, {:#06x}", rp, data)
            }
            Instruction::LoadAccumDirect(addr) => write!(f, "LDA     {:#06x}", addr),
            Instruction::StoreAccumDirect(addr) => write!(f, "STA     {:#06x}", addr),
            Instruction::LoadHLDirect(addr) => write!(f, "LHLD    {:#06x}", addr),
            Instruction::StoreHLDirect(addr) => write!(f, "SHLD    {:#06x}", addr),
            Instruction::LoadAccumIndirect(rp) => write!(f, "LDAX    {}", rp),
            Instruction::StoreAccumIndirect(rp) => write!(f, "STAX    {}", rp),
            Instruction::ExchangeHLWithDE => write!(f, "XCHG"),
            Instruction::Add(r) => write!(f, "ADD     {}", r),
            Instruction::AddImmediate(data) => write!(f, "ADI     {:#04x}", data),
            Instruction::AddWithCarry(r) => write!(f, "ADC     {}", r),
            Instruction::AddImmediateWithCarry(data) => write!(f, "ACI     {:#04x}", data),
            Instruction::Subtract(r) => write!(f, "SUB     {}", r),
            Instruction::SubtractImmediate(data) => write!(f, "SUI     {:#04x}", data),
            Instruction::SubtractWithBorrow(r) => write!(f, "SBB     {}", r),
            Instruction::SubtractImmediateWithBorrow(data) => write!(f, "SBI     {:#04x}", data),
            Instruction::Increment(r) => write!(f, "INR     {}", r),
            Instruction::Decrement(r) => write!(f, "DCR     {}", r),
            Instruction::IncrementRegPair(rp) => write!(f, "INX     {}", rp),
            Instruction::DecrementRegPair(rp) => write!(f, "DCX     {}", rp),
            Instruction::AddRegPairToHL(rp) => write!(f, "DAD     {}", rp),
            Instruction::DecimalAdjustAccum => write!(f, "DAA"),
            Instruction::And(r) => write!(f, "ANA     {}", r),
            Instruction::AndImmediate(data) => write!(f, "ANI     {:#04x}", data),
            Instruction::Xor(r) => write!(f, "XRA     {}", r),
            Instruction::XorImmediate(data) => write!(f, "XRI     {:#04x}", data),
            Instruction::Or(r) => write!(f, "ORA     {}", r),
            Instruction::OrImmediate(data) => write!(f, "ORI     {:#04x}", data),
            Instruction::Compare(r) => write!(f, "CMP     {}", r),
            Instruction::CompareImmediate(data) => write!(f, "CPI     {:#04x}", data),
            Instruction::RotateLeft => write!(f, "RLC"),
            Instruction::RotateRight => write!(f, "RRC"),
            Instruction::RotateLeftThroughCarry => write!(f, "RAL"),
            Instruction::RotateRightThroughCarry => write!(f, "RAR"),
            Instruction::ComplementAccum => write!(f, "CMA"),
            Instruction::ComplementCarry => write!(f, "CMC"),
            Instruction::SetCarry => write!(f, "STC"),
            Instruction::Jump(addr) => write!(f, "JMP     {:#06x}", addr),
            Instruction::ConditionalJump(c, addr) => write!(f, "J{}     {:#06x}", c, addr),
            Instruction::Call(addr) => write!(f, "CALL    {:#06x}", addr),
            Instruction::ConditionalCall(c, addr) => write!(f, "C{}     {:#06x}", c, addr),
            Instruction::Return => write!(f, "RET"),
            Instruction::ConditionalReturn(c) => write!(f, "J{}", c),
            Instruction::Restart(n) => write!(f, "RST     {:#04x}", n),
            Instruction::JumpHLIndirect => write!(f, "PCHL"),
            Instruction::PushRegPair(rp) => write!(f, "PUSH    {}", rp),
            Instruction::PushPSW => write!(f, "PUSH    PSW"),
            Instruction::PopRegPair(rp) => write!(f, "POP     {}", rp),
            Instruction::PopPSW => write!(f, "POP     PSW"),
            Instruction::ExchangeStackTopWithHL => write!(f, "XTHL"),
            Instruction::MoveHLToSP => write!(f, "SPHL"),
            Instruction::Input(port) => write!(f, "IN      {:#04x}", port),
            Instruction::Output(port) => write!(f, "OUT     {:#04x}", port),
            Instruction::EnableInterrupts => write!(f, "EI"),
            Instruction::DisableInterrupts => write!(f, "DI"),
            Instruction::Halt => write!(f, "HLT"),
            Instruction::NoOp => write!(f, "NOP"),
        }
    }
}
