use chip_eight::EmulatorState;
use iced::{
    Length,
    widget::{Column, Image, button, column, image::Handle, row, text},
};

use crate::{ApplicationState, Message, keypad::keypad};

pub fn application_view(app_state: &'_ ApplicationState) -> Column<'_, Message> {
    let EmulatorState {
        stack,
        variable_registers,
        index_register,
        program_counter,
        last_instruction,
        screen_buffer,
        width,
        height,
        ..
    } = &app_state.emulator_state;

    let pixels = screen_buffer
        .iter()
        .flat_map(|px| {
            if *px > 0 {
                [0xFF, 0xFF, 0xFF, 0xFF]
            } else {
                [0x0, 0x0, 0x0, 0xFF]
            }
        })
        .collect::<Vec<u8>>();

    let image_handle =
        iced::widget::image::Handle::from_rgba(*width as u32, *height as u32, pixels);
    let image_component: Image<Handle> = iced::widget::image(image_handle)
        .width(Length::Fixed(512.0))
        .height(Length::Fixed(256.0));

    let stack = format!("STACK:              {:?}", stack);
    let variable_registers = format!("VARIABLE REGISTERS: {:?}", variable_registers);
    let index_register = format!("INDEX REGISTER:     {:?}", index_register);
    let program_counter = format!("PROGRAM_COUNTER:    {:?}", program_counter);
    let last_instruction = format!("LAST_INSTRUCTION:   {:?}", last_instruction);

    let buttons = row![
        button("Next").on_press(Message::NextInstruction),
        button("Run/Stop").on_press(Message::ToggleRunning),
        button("Load Program").on_press(Message::TempLoadProgram),
    ]
    .spacing(5.0);

    column![
        text(stack),
        text(variable_registers),
        text(index_register),
        text(program_counter),
        text(last_instruction),
        buttons,
        image_component,
        keypad(app_state)
    ]
    .spacing(10.0)
}
