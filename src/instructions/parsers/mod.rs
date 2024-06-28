use arithmetic::parse_arithmetic_instruction;
use branch::parse_branch_instruction;
use data_transfer::parse_data_transfer_instruction;
use logical::parse_logical_instruction;
use machine_control::parse_machine_control_instruction;
use nom::{
    bits::{bits, complete::tag},
    branch::alt,
    bytes::complete::take,
    combinator::map,
    error::Error,
    IResult,
};

use super::{Condition, Instruction, Register, RegisterPair};

mod arithmetic;
mod branch;
mod data_transfer;
mod logical;
mod machine_control;

fn parse_u8(input: &[u8]) -> IResult<&[u8], u8> {
    map(take(1u8), |data: &[u8]| data[0])(input)
}

fn parse_u16(input: &[u8]) -> IResult<&[u8], u16> {
    map(take(2u8), |bytes: &[u8]| {
        u16::from_le_bytes([bytes[0], bytes[1]])
    })(input)
}

fn parse_byte_tag(opcode: u8) -> impl Fn(&[u8]) -> IResult<&[u8], ()> {
    move |input| {
        map(
            bits(tag::<_, _, _, Error<(&[u8], usize)>>(opcode, 8u8)),
            |_| (),
        )(input)
    }
}

fn parse_register(input: (&[u8], usize)) -> IResult<(&[u8], usize), Register> {
    alt((
        map(tag(0b111u8, 3u8), |_| Register::A),
        map(tag(0b000u8, 3u8), |_| Register::B),
        map(tag(0b001u8, 3u8), |_| Register::C),
        map(tag(0b010u8, 3u8), |_| Register::D),
        map(tag(0b011u8, 3u8), |_| Register::E),
        map(tag(0b100u8, 3u8), |_| Register::H),
        map(tag(0b101u8, 3u8), |_| Register::L),
    ))(input)
}

fn parse_register_pair(input: (&[u8], usize)) -> IResult<(&[u8], usize), RegisterPair> {
    alt((
        map(tag(0b00u8, 2u8), |_| RegisterPair::BC),
        map(tag(0b01u8, 2u8), |_| RegisterPair::DE),
        map(tag(0b10u8, 2u8), |_| RegisterPair::HL),
        map(tag(0b11u8, 2u8), |_| RegisterPair::SP),
    ))(input)
}

fn parse_condition(input: (&[u8], usize)) -> IResult<(&[u8], usize), Condition> {
    alt((
        map(tag(0b000u8, 3u8), |_| Condition::NotZero),
        map(tag(0b001u8, 3u8), |_| Condition::Zero),
        map(tag(0b010u8, 3u8), |_| Condition::NoCarry),
        map(tag(0b011u8, 3u8), |_| Condition::Carry),
        map(tag(0b100u8, 3u8), |_| Condition::OddParity),
        map(tag(0b101u8, 3u8), |_| Condition::EvenParity),
        map(tag(0b110u8, 3u8), |_| Condition::Plus),
        map(tag(0b111u8, 3u8), |_| Condition::Minus),
    ))(input)
}

pub fn parse_instruction(code: &[u8]) -> IResult<&[u8], Instruction> {
    alt((
        map(parse_data_transfer_instruction, Instruction::DataTransfer),
        map(parse_arithmetic_instruction, Instruction::Arithmetic),
        map(parse_logical_instruction, Instruction::Logical),
        map(parse_branch_instruction, Instruction::Branch),
        map(
            parse_machine_control_instruction,
            Instruction::MachineControl,
        ),
    ))(code)
}

#[cfg(test)]
mod tests {
    use nom::multi::many0;

    use super::*;

    #[test]
    fn should_parse_register() {
        let input: Vec<u8> = vec![0b111, 0b000, 0b001, 0b010, 0b011, 0b100, 0b101]
            .into_iter()
            .map(|x| x << 5)
            .collect();

        let result: IResult<&[u8], Vec<Register>> = many0(bits(parse_register))(&input);

        assert_eq!(
            result,
            Ok((
                &[] as &[u8],
                vec![
                    Register::A,
                    Register::B,
                    Register::C,
                    Register::D,
                    Register::E,
                    Register::H,
                    Register::L
                ]
            ))
        )
    }

    #[test]
    fn should_parse_register_pair() {
        let input: Vec<u8> = vec![0b00, 0b01, 0b10, 0b11]
            .into_iter()
            .map(|x| x << 6)
            .collect();

        let result: IResult<&[u8], Vec<RegisterPair>> = many0(bits(parse_register_pair))(&input);

        assert_eq!(
            result,
            Ok((
                &[] as &[u8],
                vec![
                    RegisterPair::BC,
                    RegisterPair::DE,
                    RegisterPair::HL,
                    RegisterPair::SP
                ]
            ))
        )
    }

    #[test]
    fn should_parse_condition() {
        let input: Vec<u8> = vec![0b000, 0b001, 0b010, 0b011, 0b100, 0b101, 0b110, 0b111]
            .into_iter()
            .map(|x| x << 5)
            .collect();

        let result: IResult<&[u8], Vec<Condition>> = many0(bits(parse_condition))(&input);

        assert_eq!(
            result,
            Ok((
                &[] as &[u8],
                vec![
                    Condition::NotZero,
                    Condition::Zero,
                    Condition::NoCarry,
                    Condition::Carry,
                    Condition::OddParity,
                    Condition::EvenParity,
                    Condition::Plus,
                    Condition::Minus
                ]
            ))
        )
    }
}
