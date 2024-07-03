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
    let aux_carry = ((x & 0x10) ^ (y & 0x10) ^ (result & 0x10)) != 0;

    (result, carry, aux_carry)
}

fn add_with_carry(x: u8, y: u8, carry: bool) -> (u8, bool, bool) {
    let (r_1, c_1, ac_1) = add(x, y);
    let (r_2, c_2, ac_2) = add(r_1, if carry { 1 } else { 0 });

    (r_2, c_1 || c_2, ac_1 || ac_2)
}

fn sub(x: u8, y: u8) -> (u8, bool, bool) {
    let (twos_comp, c_1, ac_1) = add(!y, 1);
    let (result, c_2, ac_2) = add(x, twos_comp);

    (result, !(c_1 || c_2), ac_1 || ac_2)
}

fn sub_with_borrow(x: u8, y: u8, carry: bool) -> (u8, bool, bool) {
    if !carry {
        return sub(x, y);
    }

    let (y_with_carry, c_1, ac_1) = add(y, 1);
    let (twos_comp, c_2, ac_2) = add(!y_with_carry, 1);
    let (result, c_3, ac_3) = add(x, twos_comp);

    (result, !(c_1 || c_2 || c_3), ac_1 || ac_2 || ac_3)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_add_correctly() {
        let (result, carry, aux_carry) = add(0x04, 0xFD);

        assert_eq!(result, 0x01);
        assert!(carry);
        assert!(aux_carry);
    }

    #[test]
    fn should_add_with_carry_correctly() {
        let (result, carry, aux_carry) = add_with_carry(0x3d, 0x42, true);

        assert_eq!(result, 0x80);
        assert!(!carry);
        assert!(aux_carry);
    }

    #[test]
    fn should_subtract_correctly() {
        let (result, carry, aux_carry) = sub(0x0c, 0x0f);

        assert_eq!(result, 0xfd);
        assert!(carry);
        assert!(!aux_carry);
    }

    #[test]
    fn should_subtract_with_borrow_correctly() {
        let (result, carry, aux_carry) = sub_with_borrow(0x04, 0x02, true);

        assert_eq!(result, 0x01);
        assert!(!carry);
        assert!(aux_carry);
    }
}
