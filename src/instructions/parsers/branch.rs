use nom::{
    bits::{
        bits,
        complete::{tag, take},
    },
    branch::alt,
    combinator::map,
    error::Error,
    sequence::{delimited, preceded, tuple},
    IResult,
};

use crate::instructions::BranchInstruction as BI;

use super::{parse_byte_tag, parse_condition, parse_u16};

pub fn parse_branch_instruction(input: &[u8]) -> IResult<&[u8], BI> {
    alt((
        // Jump
        map(preceded(parse_byte_tag(0b11000011), parse_u16), BI::Jump),
        // ConditionalJump
        map(
            tuple((
                bits(delimited(
                    tag(0b11u8, 2u8),
                    parse_condition,
                    tag(0b010u8, 3u8),
                )),
                parse_u16,
            )),
            |(condition, address)| BI::ConditionalJump(condition, address),
        ),
        // Call
        map(preceded(parse_byte_tag(0b11001101), parse_u16), BI::Call),
        // ConditionalCall
        map(
            tuple((
                bits(delimited(
                    tag(0b11u8, 2u8),
                    parse_condition,
                    tag(0b100, 3u8),
                )),
                parse_u16,
            )),
            |(condition, address)| BI::ConditionalCall(condition, address),
        ),
        // Return
        map(parse_byte_tag(0b11001001), |_| BI::Return),
        // ConditionalReturn
        map(
            bits(delimited(
                tag(0b11u8, 2u8),
                parse_condition,
                tag(0b000, 3u8),
            )),
            BI::ConditionalReturn,
        ),
        // Restart
        map(
            bits(delimited(
                tag(0b11u8, 2u8),
                take::<_, _, _, Error<(&[u8], usize)>>(3u8),
                tag(0b111u8, 3u8),
            )),
            BI::Restart,
        ),
        // JumpHLIndirect
        map(parse_byte_tag(0b11101001), |_| BI::JumpHLIndirect),
    ))(input)
}

#[cfg(test)]
mod tests {
    use nom::multi::many0;

    use crate::instructions::Condition;

    use super::*;

    #[test]
    fn should_parse_all_branch_instructions() {
        let input: &[u8] = &[
            0b11000011,
            0xAB,
            0xCD,
            0b11_010_010,
            0x12,
            0x43,
            0b11001101,
            0x55,
            0x44,
            0b11_000_100,
            0x01,
            0x99,
            0b11001001,
            0b11_110_000,
            0b11_010_111,
            0b11101001,
        ];

        let expected = vec![
            BI::Jump(0xCDAB),
            BI::ConditionalJump(Condition::NoCarry, 0x4312),
            BI::Call(0x4455),
            BI::ConditionalCall(Condition::NotZero, 0x9901),
            BI::Return,
            BI::ConditionalReturn(Condition::Plus),
            BI::Restart(0b00000_010),
            BI::JumpHLIndirect,
        ];

        let result = many0(parse_branch_instruction)(input);

        assert_eq!(result, Ok((&[] as &[u8], expected)))
    }
}
