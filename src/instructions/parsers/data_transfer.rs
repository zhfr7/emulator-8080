use nom::{
    bits::{bits, complete::tag},
    branch::alt,
    combinator::map,
    sequence::{delimited, preceded, tuple},
    IResult,
};

use crate::instructions::DataTransferInstruction as DTI;

use super::{parse_byte_tag, parse_register, parse_register_pair, parse_u16, parse_u8};

pub fn parse_data_transfer_instruction(code: &[u8]) -> IResult<&[u8], DTI> {
    alt((
        // MoveRegReg
        map(
            bits(preceded(
                tag(0b01u8, 2u8),
                tuple((parse_register, parse_register)),
            )),
            |(destination, source)| DTI::Move(source, destination),
        ),
        // MoveRegMem
        map(
            bits(preceded(tag(0b01110u8, 5u8), parse_register)),
            DTI::MoveToMem,
        ),
        // MoveMemReg
        map(
            bits(delimited(
                tag(0b01u8, 2u8),
                parse_register,
                tag(0b110u8, 3u8),
            )),
            DTI::MoveFromMem,
        ),
        // MoveImmediateReg
        map(
            tuple((
                bits(delimited(
                    tag(0b00u8, 2u8),
                    parse_register,
                    tag(0b110u8, 3u8),
                )),
                parse_u8,
            )),
            |(register, data)| DTI::MoveImmediate(register, data),
        ),
        // MoveImmediateMem
        map(
            preceded(parse_byte_tag(0b00110110), parse_u8),
            DTI::MoveToMemImmediate,
        ),
        // LoadRegisterPairImmediate
        map(
            tuple((
                bits(delimited(
                    tag(0b00u8, 2u8),
                    parse_register_pair,
                    tag(0b0001, 4u8),
                )),
                parse_u16,
            )),
            |(register_pair, data)| DTI::LoadRegisterPairImmediate(register_pair, data),
        ),
        // LoadAccumDirect
        map(
            preceded(parse_byte_tag(0b00111010), parse_u16),
            DTI::LoadAccumDirect,
        ),
        // StoreAccumDirect
        map(
            preceded(parse_byte_tag(0b00110010), parse_u16),
            DTI::StoreAccumDirect,
        ),
        // LoadHLDirect
        map(
            preceded(parse_byte_tag(0b00101010), parse_u16),
            DTI::LoadHLDirect,
        ),
        // StoreHLDirect
        map(
            preceded(parse_byte_tag(0b00100010), parse_u16),
            DTI::StoreHLDirect,
        ),
        // LoadAccumIndirect
        map(
            bits(delimited(
                tag(0, 2u8),
                parse_register_pair,
                tag(0b1010u8, 4u8),
            )),
            DTI::LoadAccumIndirect,
        ),
        // StoreAccumIndirect
        map(
            bits(delimited(
                tag(0, 2u8),
                parse_register_pair,
                tag(0b0010u8, 4u8),
            )),
            DTI::StoreAccumIndirect,
        ),
        // ExchangeHLWithDE
        map(parse_byte_tag(0b11101011), |_| DTI::ExchangeHLWithDE),
    ))(code)
}

#[cfg(test)]
mod tests {
    use std::vec;

    use nom::multi::many0;

    use crate::instructions::{Register, RegisterPair};

    use super::*;

    #[test]
    fn should_parse_all_data_transfer_instructions() {
        let input: &[u8] = &[
            0b01_010_001,
            0b01_011_110,
            0b01110_111,
            0b00_000_110,
            0xAB,
            0b00110110,
            0x1F,
            0b00_01_0001,
            0xEE,
            0x43,
            0b00111010,
            0x92,
            0x75,
            0b00110010,
            0x88,
            0xFE,
            0b00101010,
            0x01,
            0xFE,
            0b00100010,
            0x83,
            0x34,
            0b00_00_1010,
            0b00_01_0010,
            0b11101011,
        ];

        let expected = vec![
            DTI::Move(Register::C, Register::D),
            DTI::MoveFromMem(Register::E),
            DTI::MoveToMem(Register::A),
            DTI::MoveImmediate(Register::B, 0xAB),
            DTI::MoveToMemImmediate(0x1F),
            DTI::LoadRegisterPairImmediate(RegisterPair::DE, 0x43EE),
            DTI::LoadAccumDirect(0x7592),
            DTI::StoreAccumDirect(0xFE88),
            DTI::LoadHLDirect(0xFE01),
            DTI::StoreHLDirect(0x3483),
            DTI::LoadAccumIndirect(RegisterPair::BC),
            DTI::StoreAccumIndirect(RegisterPair::DE),
            DTI::ExchangeHLWithDE,
        ];

        let result = many0(parse_data_transfer_instruction)(input);

        assert_eq!(result, Ok((&[] as &[u8], expected)))
    }
}
