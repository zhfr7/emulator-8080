use std::{
    fs::{self, File},
    io::Write,
};

use emulator_8080::instructions::parsers::parse_instruction;
use nom::multi::many0;

fn main() -> std::io::Result<()> {
    let mut f = File::create("dump.txt")?;
    let input = fs::read("./roms/invaders.rom").expect("Cannot open ROM file!");

    let (_, instructions) = many0(parse_instruction)(&input)
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid data!"))?;

    for instruction in instructions {
        writeln!(f, "{}", instruction)?;
    }

    Ok(())
}
