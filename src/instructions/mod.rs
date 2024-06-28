pub mod display;
pub mod parsers;
pub mod timing;

/// Possible registers for the 8080 processor
///
/// Represented in instructions as 3-bit values
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum Register {
    /// 111
    A,
    /// 000
    B,
    /// 001
    C,
    /// 010
    D,
    /// 011
    E,
    /// 100
    H,
    /// 101
    L,
}

/// Possible register pairs used in certain instructions
///
/// Represented in instructions as 2-bit values
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum RegisterPair {
    /// 00
    BC,
    /// 01
    DE,
    /// 10
    HL,
    /// 11, stack pointer
    SP,
}

/// Possible types of conditions used in branch instructions
///
/// Represented in instructions as 3-bit values
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum Condition {
    /// 000
    NotZero,
    /// 001
    Zero,
    /// 010
    NoCarry,
    /// 011
    Carry,
    /// 100
    OddParity,
    /// 101
    EvenParity,
    /// 110
    Plus,
    /// 111
    Minus,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum DataTransferInstruction {
    Move(Register, Register),
    MoveFromMem(Register),
    MoveToMem(Register),
    MoveImmediate(Register, u8),
    MoveToMemImmediate(u8),
    LoadRegisterPairImmediate(RegisterPair, u16),
    LoadAccumDirect(u16),
    StoreAccumDirect(u16),
    LoadHLDirect(u16),
    StoreHLDirect(u16),
    LoadAccumIndirect(RegisterPair),
    StoreAccumIndirect(RegisterPair),
    ExchangeHLWithDE,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum ArithmeticInstruction {
    Add(Register),
    AddMem,
    AddImmediate(u8),
    AddWithCarry(Register),
    AddMemWithCarry,
    AddImmediateWithCarry(u8),
    Subtract(Register),
    SubtractMem,
    SubtractImmediate(u8),
    SubtractWithBorrow(Register),
    SubtractMemWithBorrow,
    SubtractImmediateWithBorrow(u8),
    Increment(Register),
    IncrementMem,
    Decrement(Register),
    DecrementMem,
    IncrementRegPair(RegisterPair),
    DecrementRegPair(RegisterPair),
    AddRegPairToHL(RegisterPair),
    DecimalAdjustAccum,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum LogicalInstruction {
    And(Register),
    AndMem,
    AndImmediate(u8),
    Xor(Register),
    XorMem,
    XorImmediate(u8),
    Or(Register),
    OrMem,
    OrImmediate(u8),
    Compare(Register),
    CompareMem,
    CompareImmediate(u8),
    RotateLeft,
    RotateRight,
    RotateLeftThroughCarry,
    RotateRightThroughCarry,
    ComplementAccum,
    ComplementCarry,
    SetCarry,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum BranchInstruction {
    Jump(u16),
    ConditionalJump(Condition, u16),
    Call(u16),
    ConditionalCall(Condition, u16),
    Return,
    ConditionalReturn(Condition),
    Restart(u8),
    JumpHLIndirect,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum MachineControlInstruction {
    PushRegPair(RegisterPair),
    PushPSW,
    PopRegPair(RegisterPair),
    PopPSW,
    ExchangeStackTopWithHL,
    MoveHLToSP,
    Input(u8),
    Output(u8),
    EnableInterrupts,
    DisableInterrupts,
    Halt,
    NoOp,
}

/// Instruction for the 8080 processor
///
/// Instructions are categorized into 5 groups:
/// - Data transfer
/// - Arithmetic
/// - Logical
/// - Branch
/// - Stack, IO, machine control
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum Instruction {
    DataTransfer(DataTransferInstruction),
    Arithmetic(ArithmeticInstruction),
    Logical(LogicalInstruction),
    Branch(BranchInstruction),
    MachineControl(MachineControlInstruction),
}
