use iced::{
    Element, Length,
    widget::{Column, button, column, pick_list, scrollable, text},
};

use crate::{ApplicationState, Message, ProgramPickerSource};

pub fn file_picker(app_state: &'_ ApplicationState) -> Element<'_, Message> {
    let pl = pick_list(
        [ProgramPickerSource::Disk, ProgramPickerSource::Online],
        Some(&app_state.program_source),
        Message::SetProgramPickerSource,
    );
    column![
        pl,
        match app_state.program_source {
            crate::ProgramPickerSource::Disk => file_program_picker(app_state),
            crate::ProgramPickerSource::Online => online_program_picker(app_state),
        }
    ]
    .spacing(5)
    .into()
}

fn file_program_picker(app_state: &'_ ApplicationState) -> Element<'_, Message> {
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

const PROGRAMS: [(&str, &str); 4] = [
    (
        "Snake",
        "https://johnearnest.github.io/chip8Archive/roms/snake.ch8",
    ),
    (
        "Rockto",
        "https://johnearnest.github.io/chip8Archive/roms/rockto.ch8",
    ),
    (
        "Tic-Tac-Toe",
        "https://johnearnest.github.io/chip8Archive/roms/ultimatetictactoe.ch8",
    ),
    (
        "Br8kout",
        "https://johnearnest.github.io/chip8Archive/roms/br8kout.ch8",
    ),
];

fn online_program_picker(app_state: &'_ ApplicationState) -> Element<'_, Message> {
    // TODO: Also add a text box to add any random url.
    let fetching = app_state.fetching_data;
    scrollable(
        Column::from_iter(PROGRAMS.map(|(name, url)| {
            let btn = button(name)
                .padding(8)
                .style(button::secondary)
                .width(Length::Fill);
            if fetching {
                btn
            } else {
                btn.on_press(Message::LoadProgramFromOnline(url.to_owned()))
            }
            .into()
        }))
        .spacing(5),
    )
    .into()
}
