use iced::{
    Alignment::Center,
    Element,
    Length::Fill,
    widget::{button, column, scrollable, text},
};

use crate::{ApplicationState, Message};

pub fn file_picker(app_state: &'_ ApplicationState) -> Element<'_, Message> {
    let button = |label, message| {
        button(text(label).width(Fill).align_x(Center).size(16))
            .width(Fill)
            .padding(8)
            .on_press(message)
    };

    let up_a_dir_button = app_state.parent_dir.as_ref().map(|parent_dir| {
        Element::from(button(
            "..".to_owned(),
            Message::EnterDirectory(parent_dir.to_owned()),
        ))
    });

    column![
        up_a_dir_button,
        Some(scrollable(
            column(app_state.current_dir.iter().map(|entry| {
                let btn_lbl = entry
                    .file_name()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or("invalid".to_string());
                if entry.is_dir() {
                    button(btn_lbl, Message::EnterDirectory(entry.to_owned())).into()
                } else {
                    button(btn_lbl, Message::LoadProgram(entry.to_owned())).into()
                }
            }))
            .spacing(5)
            .padding(5)
        ))
    ]
    .into()
}
