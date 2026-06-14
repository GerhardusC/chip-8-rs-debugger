use iced::widget::{Column, button, column, row};

use crate::{ApplicationState, Message};

pub fn keypad(_app_state: &'_ ApplicationState) -> Column<'_, Message> {
    column![
        row![
            button("1").on_press(Message::KeyPressed(1)),
            button("2").on_press(Message::KeyPressed(2)),
            button("3").on_press(Message::KeyPressed(3)),
            button("C").on_press(Message::KeyPressed(0xC)),
        ]
        .spacing(5.0),
        row![
            button("4").on_press(Message::KeyPressed(4)),
            button("5").on_press(Message::KeyPressed(5)),
            button("6").on_press(Message::KeyPressed(6)),
            button("D").on_press(Message::KeyPressed(0xD)),
        ]
        .spacing(5.0),
        row![
            button("7").on_press(Message::KeyPressed(1)),
            button("8").on_press(Message::KeyPressed(2)),
            button("9").on_press(Message::KeyPressed(3)),
            button("E").on_press(Message::KeyPressed(0xC)),
        ]
        .spacing(5.0),
        row![
            button("A").on_press(Message::KeyPressed(0xA)),
            button("0").on_press(Message::KeyPressed(0)),
            button("B").on_press(Message::KeyPressed(0xB)),
            button("F").on_press(Message::KeyPressed(0xF)),
        ]
        .spacing(5.0),
    ]
    .spacing(5.0)
}
