use arithmetic::execute_arithmetic_instruction;
use branch::execute_branch_instruction;
use data_transfer::execute_data_transfer_instruction;
use logical::execute_logical_instruction;
use machine_control::execute_machine_control_instruction;

use crate::{instructions::Instruction, state::State};

mod arithmetic;
mod branch;
mod data_transfer;
mod logical;
mod machine_control;

fn add(x: u8, y: u8) -> (u8, bool, bool) {
    let (result, carry) = x.overflowing_add(y);
    let aux_carry = (x & 15).wrapping_add(y & 15) > 16;

    (result, carry, aux_carry)
}

fn add_with_carry(x: u8, y: u8, carry: bool) -> (u8, bool, bool) {
    let (r_1, c_1, ac_1) = add(x, y);
    let (r_2, c_2, ac_2) = add(r_1, if carry { 1 } else { 0 });

    (r_2, c_1 || c_2, ac_1 || ac_2)
}

fn sub(x: u8, y: u8) -> (u8, bool, bool) {
    let (result, carry, aux_carry) = add_with_carry(x, !y, true);

    (result, !carry, aux_carry)
}

fn sub_with_borrow(x: u8, y: u8, carry: bool) -> (u8, bool, bool) {
    let (r_1, c_1, ac_1) = add_with_carry(!y, 1, carry);
    let (r_2, c_2, ac_2) = add(x, r_1);

    (r_2, !(c_1 || c_2), ac_1 || ac_2)
}

pub fn execute_instruction(state: &mut State, instruction: &Instruction) {
    match instruction {
        Instruction::DataTransfer(dti) => execute_data_transfer_instruction(state, dti),
        Instruction::Arithmetic(ai) => execute_arithmetic_instruction(state, ai),
        Instruction::Logical(li) => execute_logical_instruction(state, li),
        Instruction::Branch(bi) => execute_branch_instruction(state, bi),
        Instruction::MachineControl(mci) => execute_machine_control_instruction(state, mci),
    }
}
