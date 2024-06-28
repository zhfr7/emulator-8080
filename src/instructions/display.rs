use std::fmt::Display;

use super::{
    ArithmeticInstruction as AI, BranchInstruction as BI, Condition,
    DataTransferInstruction as DTI, Instruction, LogicalInstruction as LI,
    MachineControlInstruction as MCI, Register, RegisterPair,
};

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
        match self {
            Instruction::DataTransfer(i) => match i {
                DTI::Move(r2, r1) => write!(f, "MOV     {}, {}", r1, r2),
                DTI::MoveFromMem(r) => write!(f, "MOV     {}, M", r),
                DTI::MoveToMem(r) => write!(f, "MOV     M, {}", r),
                DTI::MoveImmediate(r, data) => write!(f, "MVI     {}, {:#04x}", r, data),
                DTI::MoveToMemImmediate(data) => write!(f, "MVI     M, {:#04x}", data),
                DTI::LoadRegisterPairImmediate(rp, data) => {
                    write!(f, "LXI     {}, {:#06x}", rp, data)
                }
                DTI::LoadAccumDirect(addr) => write!(f, "LDA     {:#06x}", addr),
                DTI::StoreAccumDirect(addr) => write!(f, "STA     {:#06x}", addr),
                DTI::LoadHLDirect(addr) => write!(f, "LHLD    {:#06x}", addr),
                DTI::StoreHLDirect(addr) => write!(f, "SHLD    {:#06x}", addr),
                DTI::LoadAccumIndirect(rp) => write!(f, "LDAX    {}", rp),
                DTI::StoreAccumIndirect(rp) => write!(f, "STAX    {}", rp),
                DTI::ExchangeHLWithDE => write!(f, "XCHG"),
            },
            Instruction::Arithmetic(i) => match i {
                AI::Add(r) => write!(f, "ADD     {}", r),
                AI::AddMem => write!(f, "ADD     M"),
                AI::AddImmediate(data) => write!(f, "ADI     {:#04x}", data),
                AI::AddWithCarry(r) => write!(f, "ADC     {}", r),
                AI::AddMemWithCarry => write!(f, "ADC     M"),
                AI::AddImmediateWithCarry(data) => write!(f, "ACI     {:#04x}", data),
                AI::Subtract(r) => write!(f, "SUB     {}", r),
                AI::SubtractMem => write!(f, "SUB     M"),
                AI::SubtractImmediate(data) => write!(f, "SUI     {:#04x}", data),
                AI::SubtractWithBorrow(r) => write!(f, "SBB     {}", r),
                AI::SubtractMemWithBorrow => write!(f, "SBB     M"),
                AI::SubtractImmediateWithBorrow(data) => write!(f, "SBI     {:#04x}", data),
                AI::Increment(r) => write!(f, "INR     {}", r),
                AI::IncrementMem => write!(f, "INR     M"),
                AI::Decrement(r) => write!(f, "DCR     {}", r),
                AI::DecrementMem => write!(f, "DCR     M"),
                AI::IncrementRegPair(rp) => write!(f, "INX     {}", rp),
                AI::DecrementRegPair(rp) => write!(f, "DCX     {}", rp),
                AI::AddRegPairToHL(rp) => write!(f, "DAD     {}", rp),
                AI::DecimalAdjustAccum => write!(f, "DAA"),
            },
            Instruction::Logical(i) => match i {
                LI::And(r) => write!(f, "ANA     {}", r),
                LI::AndMem => write!(f, "ANA     M"),
                LI::AndImmediate(data) => write!(f, "ANI     {:#04x}", data),
                LI::Xor(r) => write!(f, "XRA     {}", r),
                LI::XorMem => write!(f, "XRA     M"),
                LI::XorImmediate(data) => write!(f, "XRI     {:#04x}", data),
                LI::Or(r) => write!(f, "ORA     {}", r),
                LI::OrMem => write!(f, "ORA     M"),
                LI::OrImmediate(data) => write!(f, "ORI     {:#04x}", data),
                LI::Compare(r) => write!(f, "CMP     {}", r),
                LI::CompareMem => write!(f, "CMP     M"),
                LI::CompareImmediate(data) => write!(f, "CPI     {:#04x}", data),
                LI::RotateLeft => write!(f, "RLC"),
                LI::RotateRight => write!(f, "RRC"),
                LI::RotateLeftThroughCarry => write!(f, "RAL"),
                LI::RotateRightThroughCarry => write!(f, "RAR"),
                LI::ComplementAccum => write!(f, "CMA"),
                LI::ComplementCarry => write!(f, "CMC"),
                LI::SetCarry => write!(f, "STC"),
            },
            Instruction::Branch(i) => match i {
                BI::Jump(addr) => write!(f, "JMP     {:#06x}", addr),
                BI::ConditionalJump(c, addr) => write!(f, "J{}     {:#06x}", c, addr),
                BI::Call(addr) => write!(f, "CALL    {:#06x}", addr),
                BI::ConditionalCall(c, addr) => write!(f, "C{}     {:#06x}", c, addr),
                BI::Return => write!(f, "RET"),
                BI::ConditionalReturn(c) => write!(f, "J{}", c),
                BI::Restart(n) => write!(f, "RST     {:#04x}", n),
                BI::JumpHLIndirect => write!(f, "PCHL"),
            },
            Instruction::MachineControl(i) => match i {
                MCI::PushRegPair(rp) => write!(f, "PUSH    {}", rp),
                MCI::PushPSW => write!(f, "PUSH    PSW"),
                MCI::PopRegPair(rp) => write!(f, "POP     {}", rp),
                MCI::PopPSW => write!(f, "POP     PSW"),
                MCI::ExchangeStackTopWithHL => write!(f, "XTHL"),
                MCI::MoveHLToSP => write!(f, "SPHL"),
                MCI::Input(port) => write!(f, "IN      {:#04x}", port),
                MCI::Output(port) => write!(f, "OUT     {:#04x}", port),
                MCI::EnableInterrupts => write!(f, "EI"),
                MCI::DisableInterrupts => write!(f, "DI"),
                MCI::Halt => write!(f, "HLT"),
                MCI::NoOp => write!(f, "NOP"),
            },
        }
    }
}
