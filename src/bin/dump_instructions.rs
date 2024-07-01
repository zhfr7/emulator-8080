use std::{
    fs::{self, File},
    io::Write,
};

use emulator_8080::instructions::{parsers::parse_instruction, Instruction};
use nom::{multi::many0, IResult};

fn parse_instruction_with_length(input: &[u8]) -> IResult<&[u8], (Instruction, usize)> {
    parse_instruction(input)
        .map(|(rest, instruction)| (rest, (instruction, input.len() - rest.len())))
}

fn main() -> std::io::Result<()> {
    let mut f = File::create("dump.txt")?;
    let input = fs::read("./test_roms/CPUTEST.COM").expect("Cannot open ROM file!");

    let (rest, instructions) = many0(parse_instruction_with_length)(&input)
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid data!"))?;

    println!("{:?}", &rest);

    let mut address = 0x100;

    for (instruction, size) in instructions {
        writeln!(f, "{:#06x} {}", address, instruction)?;
        address += size;
    }

    Ok(())
}
