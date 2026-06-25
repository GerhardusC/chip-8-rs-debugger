use chip_eight::EmulatorState;
use iced::{
    Element, widget::{Column, Row, column, container, text},
};

use crate::{ApplicationState, Message};

pub fn metadata(app_state: &'_ ApplicationState) -> Column<'_, Message> {
    let EmulatorState {
        index_register,
        program_counter,
        ..
    } = &app_state.emulator_state;

    let index_register = format!("INDEX REGISTER:     {:?}", index_register);
    let program_counter = format!("PROGRAM_COUNTER:    {:?}", program_counter);

    column![
        variable_registers_view(app_state),
        text(index_register),
        text(program_counter),
        stack_view(app_state),
    ]
    .spacing(5)
}

fn stack_view(app_state: &'_ ApplicationState) -> Element<'_, Message> {
    if app_state.emulator_state.stack.is_empty() {
        let element: Option<Element<'_, Message>> = None;
        return element.into();
    }

    let heading = container(text("Stack:").style(text::secondary)).padding(5);

    Some(column![
        heading,
        Row::from_iter(app_state.emulator_state.stack.iter().map(|x| {
            container(text(x.to_string()))
                .style(container::bordered_box)
                .padding(10)
                .into()
        }))
        .spacing(5),
    ])
    .into()
}

fn variable_registers_view(app_state: &'_ ApplicationState) -> Column<'_, Message> {
    let heading = container(text("Variable registers:").style(text::secondary)).padding(5);

    column![
        heading,
        Row::from_iter(app_state.emulator_state.variable_registers.iter().map(|x| {
            container(text(x.to_string()))
                .style(container::bordered_box)
                .padding(10)
                .into()
        }))
        .spacing(5)
    ]
}
