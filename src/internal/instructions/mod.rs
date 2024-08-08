pub mod display;
pub mod timing;

/// Possible registers for the 8080 processor
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum Register {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    Memory,
}

/// Possible register pairs used in certain instructions
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum RegisterPair {
    BC,
    DE,
    HL,
    SP,
}

/// Possible types of conditions used in branch instructions
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum Condition {
    NotZero,
    Zero,
    NoCarry,
    Carry,
    OddParity,
    EvenParity,
    Plus,
    Minus,
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
    // Data transfer
    Move(Register, Register),
    MoveImmediate(Register, u8),
    LoadRegisterPairImmediate(RegisterPair, u16),
    LoadAccumDirect(u16),
    StoreAccumDirect(u16),
    LoadHLDirect(u16),
    StoreHLDirect(u16),
    LoadAccumIndirect(RegisterPair),
    StoreAccumIndirect(RegisterPair),
    ExchangeHLWithDE,

    // Arithmetic
    Add(Register),
    AddImmediate(u8),
    AddWithCarry(Register),
    AddImmediateWithCarry(u8),
    Subtract(Register),
    SubtractImmediate(u8),
    SubtractWithBorrow(Register),
    SubtractImmediateWithBorrow(u8),
    Increment(Register),
    Decrement(Register),
    IncrementRegPair(RegisterPair),
    DecrementRegPair(RegisterPair),
    AddRegPairToHL(RegisterPair),
    DecimalAdjustAccum,

    // Logical
    And(Register),
    AndImmediate(u8),
    Xor(Register),
    XorImmediate(u8),
    Or(Register),
    OrImmediate(u8),
    Compare(Register),
    CompareImmediate(u8),
    RotateLeft,
    RotateRight,
    RotateLeftThroughCarry,
    RotateRightThroughCarry,
    ComplementAccum,
    ComplementCarry,
    SetCarry,

    // Branch
    Jump(u16),
    ConditionalJump(Condition, u16),
    Call(u16),
    ConditionalCall(Condition, u16),
    Return,
    ConditionalReturn(Condition),
    Restart(u8),
    JumpHLIndirect,

    // Machine control
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
