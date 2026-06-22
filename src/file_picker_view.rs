use iced::{
    Element, Length,
    widget::{button, column, scrollable, text},
};

use crate::{ApplicationState, Message};

pub fn file_picker(app_state: &'_ ApplicationState) -> Element<'_, Message> {
    let button = |label, message| {
        button(text(label).size(16))
            .padding(8)
            .width(Length::Fill)
            .on_press(message)
    };

    let up_a_dir_button = app_state.parent_dir.as_ref().map(|parent_dir| {
        Element::from(
            button(
                "..".to_owned(),
                Message::EnterDirectory(parent_dir.to_owned()),
            )
            .style(button::secondary),
        )
    });
    let mut files = app_state.current_dir.clone();

    files.sort_by_key(|a| a.is_file());

    column![
        up_a_dir_button,
        Some(scrollable(
            column(files.iter().map(|entry| {
                let btn_lbl = entry
                    .file_name()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or("invalid".to_string());
                if entry.is_dir() {
                    button(btn_lbl + "/", Message::EnterDirectory(entry.to_owned()))
                        .style(button::secondary)
                        .into()
                } else {
                    button(btn_lbl, Message::LoadProgram(entry.to_owned()))
                        .style(button::subtle)
                        .into()
                }
            }))
            .spacing(5)
        ))
    ]
    .spacing(5)
    .into()
}
