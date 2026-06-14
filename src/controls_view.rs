use iced::{
    Element,
    widget::{button, row},
};

use crate::{ApplicationState, Message};

pub fn controls(_app_state: &'_ ApplicationState) -> Element<'_, Message> {
    row![
        button("Next").on_press(Message::NextInstruction),
        button("Run/Stop").on_press(Message::ToggleRunning),
        button("Load Program").on_press(Message::TempLoadProgram),
    ]
    .spacing(5.0)
    .into()
}
