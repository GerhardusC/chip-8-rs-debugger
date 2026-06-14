use iced::{
    Alignment::Center,
    Element,
    Length::Fill,
    widget::{button, column, text},
};

use crate::{ApplicationState, Message};

pub fn file_picker(app_state: &'_ ApplicationState) -> Element<'_, Message> {
    let button = |label, message| {
        button(text(label).width(Fill).align_x(Center).size(16))
            .width(Fill)
            .padding(8)
            .on_press(message)
    };

    column(app_state.current_dir.iter().map(|entry| {
        let p = entry.file_name().to_string_lossy().into_owned();
        button(p, Message::Tick).into()
    }))
    .into()
}
