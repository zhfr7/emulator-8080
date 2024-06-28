use nom::{
    bits::{bits, complete::tag},
    branch::alt,
    combinator::map,
    sequence::preceded,
    IResult,
};

use crate::instructions::LogicalInstruction as LI;

use super::{parse_byte_tag, parse_register, parse_u8};

pub fn parse_logical_instruction(code: &[u8]) -> IResult<&[u8], LI> {
    alt((
        // And
        map(bits(preceded(tag(0b10100u8, 5u8), parse_register)), LI::And),
        // AndMem
        map(parse_byte_tag(0b10100110), |_| LI::AndMem),
        // AndImmediate
        map(
            preceded(parse_byte_tag(0b11100110), parse_u8),
            LI::AndImmediate,
        ),
        // Xor
        map(bits(preceded(tag(0b10101u8, 5u8), parse_register)), LI::Xor),
        // XorMem
        map(parse_byte_tag(0b10101110), |_| LI::XorMem),
        // XorImmediate
        map(
            preceded(parse_byte_tag(0b11101110), parse_u8),
            LI::XorImmediate,
        ),
        // Or
        map(bits(preceded(tag(0b10110u8, 5u8), parse_register)), LI::Or),
        // OrMem
        map(parse_byte_tag(0b10110110), |_| LI::OrMem),
        // OrImmediate
        map(
            preceded(parse_byte_tag(0b11110110), parse_u8),
            LI::OrImmediate,
        ),
        // Compare
        map(
            bits(preceded(tag(0b10111u8, 5u8), parse_register)),
            LI::Compare,
        ),
        // CompareMem
        map(parse_byte_tag(0b10111110), |_| LI::CompareMem),
        // CompareImmediate
        map(
            preceded(parse_byte_tag(0b11111110), parse_u8),
            LI::CompareImmediate,
        ),
        // RotateLeft
        map(parse_byte_tag(0b00000111), |_| LI::RotateLeft),
        // RotateRight
        map(parse_byte_tag(0b00001111), |_| LI::RotateRight),
        // RotateLeftThroughCarry
        map(parse_byte_tag(0b00010111), |_| LI::RotateLeftThroughCarry),
        // RotateRightThroughCarry
        map(parse_byte_tag(0b00011111), |_| LI::RotateRightThroughCarry),
        // ComplementAccum
        map(parse_byte_tag(0b00101111), |_| LI::ComplementAccum),
        // ComplementCarry
        map(parse_byte_tag(0b00111111), |_| LI::ComplementCarry),
        // SetCarry
        map(parse_byte_tag(0b00110111), |_| LI::SetCarry),
    ))(code)
}

#[cfg(test)]
mod tests {
    use nom::multi::many0;

    use crate::instructions::Register;

    use super::*;

    #[test]
    fn should_parse_all_logic_instructions() {
        let input: &[u8] = &[
            0b10100_000,
            0b10100110,
            0b11100110,
            0x66,
            0b10101_000,
            0b10101110,
            0b11101110,
            0x78,
            0b10110_000,
            0b10110110,
            0b11110110,
            0x98,
            0b10111_000,
            0b10111110,
            0b11111110,
            0xA1,
            0b00000111,
            0b00001111,
            0b00010111,
            0b00011111,
            0b00101111,
            0b00111111,
            0b00110111,
        ];

        let expected = vec![
            LI::And(Register::B),
            LI::AndMem,
            LI::AndImmediate(0x66),
            LI::Xor(Register::B),
            LI::XorMem,
            LI::XorImmediate(0x78),
            LI::Or(Register::B),
            LI::OrMem,
            LI::OrImmediate(0x98),
            LI::Compare(Register::B),
            LI::CompareMem,
            LI::CompareImmediate(0xA1),
            LI::RotateLeft,
            LI::RotateRight,
            LI::RotateLeftThroughCarry,
            LI::RotateRightThroughCarry,
            LI::ComplementAccum,
            LI::ComplementCarry,
            LI::SetCarry,
        ];

        let result = many0(parse_logical_instruction)(input);

        assert_eq!(result, Ok((&[] as &[u8], expected)))
    }
}
