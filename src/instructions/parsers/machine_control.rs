use nom::{
    bits::{bits, complete::tag},
    branch::alt,
    combinator::map,
    sequence::{delimited, preceded},
    IResult,
};

use crate::instructions::MachineControlInstruction as MCI;

use super::{parse_byte_tag, parse_register_pair, parse_u8};

pub fn parse_machine_control_instruction(input: &[u8]) -> IResult<&[u8], MCI> {
    alt((
        // PushPSW
        map(parse_byte_tag(0b11110101), |_| MCI::PushPSW),
        // PushRegPair
        map(
            bits(delimited(
                tag(0b11u8, 2u8),
                parse_register_pair,
                tag(0b0101u8, 4u8),
            )),
            MCI::PushRegPair,
        ),
        // PopPSW
        map(parse_byte_tag(0b11110001), |_| MCI::PopPSW),
        // PopRegPair
        map(
            bits(delimited(
                tag(0b11u8, 2u8),
                parse_register_pair,
                tag(0b0001u8, 4u8),
            )),
            MCI::PopRegPair,
        ),
        // ExchangeStackTopWithHL
        map(parse_byte_tag(0b11100011), |_| MCI::ExchangeStackTopWithHL),
        // Move HL to SP
        map(parse_byte_tag(0b11111001), |_| MCI::MoveHLToSP),
        // Input
        map(preceded(parse_byte_tag(0b11011011), parse_u8), MCI::Input),
        // Output
        map(preceded(parse_byte_tag(0b11010011), parse_u8), MCI::Output),
        // EnableInterrupts
        map(parse_byte_tag(0b11111011), |_| MCI::EnableInterrupts),
        // DisableInterrupts
        map(parse_byte_tag(0b11110011), |_| MCI::DisableInterrupts),
        // Halt
        map(parse_byte_tag(0b01110110), |_| MCI::Halt),
        // NoOp
        map(
            alt((
                parse_byte_tag(0x00),
                parse_byte_tag(0x10),
                parse_byte_tag(0x20),
                parse_byte_tag(0x30),
                parse_byte_tag(0x08),
                parse_byte_tag(0x18),
                parse_byte_tag(0x28),
                parse_byte_tag(0x38),
            )),
            |_| MCI::NoOp,
        ),
    ))(input)
}

#[cfg(test)]
mod tests {
    use nom::multi::many0;

    use crate::instructions::RegisterPair;

    use super::*;

    #[test]
    fn should_parse_all_branch_instructions() {
        let input: &[u8] = &[
            0b11_01_0101,
            0b11110101,
            0b11_00_0001,
            0b11110001,
            0b11100011,
            0b11111001,
            0b11011011,
            0xA8,
            0b11010011,
            0x05,
            0b11111011,
            0b11110011,
            0b01110110,
            0,
        ];

        let expected = vec![
            MCI::PushRegPair(RegisterPair::DE),
            MCI::PushPSW,
            MCI::PopRegPair(RegisterPair::BC),
            MCI::PopPSW,
            MCI::ExchangeStackTopWithHL,
            MCI::MoveHLToSP,
            MCI::Input(0xA8),
            MCI::Output(0x05),
            MCI::EnableInterrupts,
            MCI::DisableInterrupts,
            MCI::Halt,
            MCI::NoOp,
        ];

        let result = many0(parse_machine_control_instruction)(input);

        assert_eq!(result, Ok((&[] as &[u8], expected)))
    }
}
