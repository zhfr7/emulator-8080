use crate::{
    instructions::{DataTransferInstruction as DTI, Register, RegisterPair},
    memory::AddressableMemory,
    state::State,
};

pub fn execute_data_transfer_instruction(state: &mut State, instruction: &DTI) {
    match instruction {
        DTI::Move(source, destination) => {
            let value = state.registers.get(source);

            state.registers.set(destination, value);
        }
        DTI::MoveFromMem(destination) => {
            let value = state.get_memory_value();

            state.registers.set(destination, value);
        }
        DTI::MoveToMem(source) => {
            let value = state.registers.get(source);

            state.set_memory_value(value);
        }
        DTI::MoveImmediate(destination, data) => {
            state.registers.set(destination, *data);
            state.increment_program_counter();
        }
        DTI::MoveToMemImmediate(data) => {
            state.set_memory_value(*data);
            state.increment_program_counter();
        }
        DTI::LoadRegisterPairImmediate(register_pair, data) => {
            state.registers.set_pair(register_pair, *data);
            state.increment_program_counter_by(2);
        }
        DTI::LoadAccumDirect(address) => {
            let value = state.memory.get(*address);

            state.registers.set(&Register::A, value);
            state.increment_program_counter_by(2);
        }
        DTI::StoreAccumDirect(address) => {
            let value = state.registers.get(&Register::A);

            state.memory.set(*address, value);
            state.increment_program_counter_by(2);
        }
        DTI::LoadHLDirect(address) => {
            let succeeding_address = address.wrapping_add(1);
            let l_value = state.memory.get(*address);
            let h_value = state.memory.get(succeeding_address);

            state.registers.set(&Register::L, l_value);
            state.registers.set(&Register::H, h_value);
            state.increment_program_counter_by(2);
        }
        DTI::StoreHLDirect(address) => {
            let succeeding_address = address.wrapping_add(1);
            let l_value = state.registers.get(&Register::L);
            let h_value = state.registers.get(&Register::H);

            state.memory.set(*address, l_value);
            state.memory.set(succeeding_address, h_value);
            state.increment_program_counter_by(2);
        }
        DTI::LoadAccumIndirect(register_pair) => {
            let address = state.registers.get_pair(register_pair);
            let value = state.memory.get(address);

            state.registers.set(&Register::A, value);
        }
        DTI::StoreAccumIndirect(register_pair) => {
            let address = state.registers.get_pair(register_pair);
            let value = state.registers.get(&Register::A);

            state.memory.set(address, value);
        }
        DTI::ExchangeHLWithDE => {
            let hl_value = state.registers.get_pair(&RegisterPair::HL);

            state.registers.set_pair(
                &RegisterPair::HL,
                state.registers.get_pair(&RegisterPair::DE),
            );
            state.registers.set_pair(&RegisterPair::DE, hl_value);
        }
    }

    state.increment_program_counter()
}
