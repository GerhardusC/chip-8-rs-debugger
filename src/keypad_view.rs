use iced::{
    Element, Length,
    widget::{Button, button, center, column, row, text},
};

use crate::{ApplicationState, Message};

fn keypad_btn(app_state: &'_ ApplicationState, value: u8) -> Button<'_, Message> {
    let btn_text = format!("{:X}", value);
    let active = app_state
        .emulator_related_data
        .emulator
        .0
        .borrow()
        .input_provider
        .keys_state
        .get(value as usize)
        .map(|x| *x > 0)
        .unwrap_or(false);
    let bton: Button<'_, Message> = button(text(btn_text))
        .on_press(Message::EmulatorKey(value))
        .style(if active {
            button::success
        } else {
            button::primary
        })
        .width(Length::Fixed(30.0));

    bton
}

pub fn keypad(app_state: &'_ ApplicationState) -> Element<'_, Message> {
    center(
        column![
            row![
                keypad_btn(app_state, 1),
                keypad_btn(app_state, 2),
                keypad_btn(app_state, 3),
                keypad_btn(app_state, 0xC),
            ]
            .spacing(5.0),
            row![
                keypad_btn(app_state, 4),
                keypad_btn(app_state, 5),
                keypad_btn(app_state, 6),
                keypad_btn(app_state, 0xD),
            ]
            .spacing(5.0),
            row![
                keypad_btn(app_state, 7),
                keypad_btn(app_state, 8),
                keypad_btn(app_state, 9),
                keypad_btn(app_state, 0xE),
            ]
            .spacing(5.0),
            row![
                keypad_btn(app_state, 0xA),
                keypad_btn(app_state, 0),
                keypad_btn(app_state, 0xB),
                keypad_btn(app_state, 0xF),
            ]
            .spacing(5.0),
        ]
        .spacing(5.0),
    )
    .into()
}
