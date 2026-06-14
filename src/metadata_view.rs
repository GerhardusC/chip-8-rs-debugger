use chip_eight::EmulatorState;
use iced::widget::{Column, column, text};

use crate::{ApplicationState, Message};

pub fn metadata(app_state: &'_ ApplicationState) -> Column<'_, Message> {
    let EmulatorState {
        stack,
        variable_registers,
        index_register,
        program_counter,
        last_instruction,
        ..
    } = &app_state.emulator_state;

    let stack = format!("STACK:              {:?}", stack);
    let variable_registers = format!("VARIABLE REGISTERS: {:?}", variable_registers);
    let index_register = format!("INDEX REGISTER:     {:?}", index_register);
    let program_counter = format!("PROGRAM_COUNTER:    {:?}", program_counter);
    let last_instruction = format!("LAST_INSTRUCTION:   {:?}", last_instruction);

    column![
        text(stack),
        text(variable_registers),
        text(index_register),
        text(program_counter),
        text(last_instruction),
    ]
}
