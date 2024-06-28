use nom::{
    bits::{bits, complete::tag},
    branch::alt,
    combinator::map,
    sequence::{delimited, preceded},
    IResult,
};

use crate::instructions::ArithmeticInstruction as AI;

use super::{parse_byte_tag, parse_register, parse_register_pair, parse_u8};

pub fn parse_arithmetic_instruction(code: &[u8]) -> IResult<&[u8], AI> {
    alt((
        // Add
        map(bits(preceded(tag(0b10000u8, 5u8), parse_register)), AI::Add),
        // AddMem
        map(parse_byte_tag(0b10000110), |_| AI::AddMem),
        // AddImmediate
        map(
            preceded(parse_byte_tag(0b11000110), parse_u8),
            AI::AddImmediate,
        ),
        // AddWithCarry
        map(
            bits(preceded(tag(0b10001u8, 5u8), parse_register)),
            AI::AddWithCarry,
        ),
        // AddMemWithCarry
        map(parse_byte_tag(0b10001110), |_| AI::AddMemWithCarry),
        // AddImmediateWithCarry
        map(
            preceded(parse_byte_tag(0b11001110), parse_u8),
            AI::AddImmediateWithCarry,
        ),
        // Subtract
        map(
            bits(preceded(tag(0b10010u8, 5u8), parse_register)),
            AI::Subtract,
        ),
        // SubtractMem
        map(parse_byte_tag(0b10010110), |_| AI::SubtractMem),
        // SubtractImmediate
        map(
            preceded(parse_byte_tag(0b11010110), parse_u8),
            AI::SubtractImmediate,
        ),
        // SubtractWithBorrow
        map(
            bits(preceded(tag(0b10011u8, 5u8), parse_register)),
            AI::SubtractWithBorrow,
        ),
        // SubtractMemoryWithBorrow
        map(parse_byte_tag(0b10011110), |_| AI::SubtractMemWithBorrow),
        // SubtractImmediateWithBorrow
        map(
            preceded(parse_byte_tag(0b11011110), parse_u8),
            AI::SubtractImmediateWithBorrow,
        ),
        // Increment
        map(
            bits(delimited(tag(0, 2u8), parse_register, tag(0b100, 3u8))),
            AI::Increment,
        ),
        // IncrementMem
        map(parse_byte_tag(0b00110100), |_| AI::IncrementMem),
        // Decrement
        map(
            bits(delimited(tag(0, 2u8), parse_register, tag(0b101, 3u8))),
            AI::Decrement,
        ),
        // DecrementMem
        map(parse_byte_tag(0b00110101), |_| AI::DecrementMem),
        // IncrementRegPair
        map(
            bits(delimited(
                tag(0, 2u8),
                parse_register_pair,
                tag(0b0011u8, 4u8),
            )),
            AI::IncrementRegPair,
        ),
        // DecrementRegPair
        map(
            bits(delimited(
                tag(0, 2u8),
                parse_register_pair,
                tag(0b1011u8, 4u8),
            )),
            AI::DecrementRegPair,
        ),
        // AddRegPairToHL
        map(
            bits(delimited(
                tag(0, 2u8),
                parse_register_pair,
                tag(0b1001u8, 4u8),
            )),
            AI::AddRegPairToHL,
        ),
        // DecimalAdjustAccum
        map(parse_byte_tag(0b00100111), |_| AI::DecimalAdjustAccum),
    ))(code)
}

#[cfg(test)]
mod tests {
    use nom::multi::many0;

    use crate::instructions::{Register, RegisterPair};

    use super::*;

    #[test]
    fn should_parse_all_arithmetic_instructions() {
        let input: &[u8] = &[
            0b10000_000,
            0b10000110,
            0b11000110,
            0xFE,
            0b10001_001,
            0b10001110,
            0b11001110,
            0x53,
            0b10010_010,
            0b10010110,
            0b11010110,
            0x77,
            0b10011_100,
            0b10011110,
            0b11011110,
            0x4F,
            0b00_101_100,
            0b00110100,
            0b00_111_101,
            0b00110101,
            0b00_10_0011,
            0b00_01_1011,
            0b00_00_1001,
            0b00100111,
        ];

        let expected = vec![
            AI::Add(Register::B),
            AI::AddMem,
            AI::AddImmediate(0xFE),
            AI::AddWithCarry(Register::C),
            AI::AddMemWithCarry,
            AI::AddImmediateWithCarry(0x53),
            AI::Subtract(Register::D),
            AI::SubtractMem,
            AI::SubtractImmediate(0x77),
            AI::SubtractWithBorrow(Register::H),
            AI::SubtractMemWithBorrow,
            AI::SubtractImmediateWithBorrow(0x4F),
            AI::Increment(Register::L),
            AI::IncrementMem,
            AI::Decrement(Register::A),
            AI::DecrementMem,
            AI::IncrementRegPair(RegisterPair::HL),
            AI::DecrementRegPair(RegisterPair::DE),
            AI::AddRegPairToHL(RegisterPair::BC),
            AI::DecimalAdjustAccum,
        ];

        let result = many0(parse_arithmetic_instruction)(input);

        assert_eq!(result, Ok((&[] as &[u8], expected)))
    }
}
