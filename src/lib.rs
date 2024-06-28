// Sources:
// - https://deramp.com/downloads/intel/8080%20Data%20Sheet.pdf
// - https://drakeor.com/uploads/8080-Programmers-Manual.pdf
// - https://github.com/superzazu/8080
// - https://altairclone.com/downloads/cpu_tests/8080_8085%20CPU%20Exerciser.pdf

pub mod instructions;
pub mod system;

mod execution;
mod memory;
mod register;
mod state;
