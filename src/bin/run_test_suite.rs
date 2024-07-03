use std::fs;

use emulator_8080::system::test::TestSystem;

fn main() {
    let test_rom = fs::read("./test_roms/CPUTEST.COM").expect("Cannot load test ROM TST8080.COM!");

    let mut system = TestSystem::new();

    system.load_test_program(test_rom);

    let mut instruction_count: usize = 0;

    while system.state.enabled {
        instruction_count += 1;
        system.run_current_instruction();
    }

    println!("Instruction count: {}", instruction_count);
}
