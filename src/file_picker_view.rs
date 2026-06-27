use std::path::PathBuf;

use iced::{
    Element, Length,
    widget::{Column, TextInput, button, column, pick_list, row, scrollable, space, text},
};

use crate::{ApplicationState, Message, ProgramPickerSource};

pub fn file_picker(app_state: &'_ ApplicationState) -> Element<'_, Message> {
    let program_source_pick_list = pick_list(
        [ProgramPickerSource::Disk, ProgramPickerSource::Online],
        Some(&app_state.program_source),
        Message::SetProgramPickerSource,
    );

    let (program_picker, on_submit, on_input, label) = match &app_state.program_source {
        ProgramPickerSource::Disk => {
            let program_path = PathBuf::from(if app_state.program_path.starts_with("~") {
                std::env!("HOME").to_owned() + app_state.program_path.trim_start_matches("~")
            } else {
                app_state.program_path.to_owned()
            });
            (
                file_program_picker(app_state),
                if program_path.is_dir() {
                    Message::EnterDirectory(program_path)
                } else {
                    Message::LoadProgram(program_path)
                },
                Message::UpdateProgramPath,
                "File or Dir",
            )
        }
        ProgramPickerSource::Online => (
            online_program_picker(app_state),
            Message::LoadProgramFromOnline(app_state.program_path.to_owned()),
            Message::UpdateProgramPath,
            "URL",
        ),
    };

    let submit_btn = button("Go");
    let input = TextInput::new(label, &app_state.program_path);
    let (input, submit_btn) = if app_state.fetching_data {
        (input, submit_btn)
    } else {
        (
            input.on_submit(on_submit.clone()).on_input(on_input),
            submit_btn.on_press(on_submit),
        )
    };
    let top_row = row![
        program_source_pick_list,
        space::horizontal(),
        input,
        submit_btn
    ]
    .spacing(5);
    column![top_row, program_picker,].spacing(5).into()
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

const DEFAULT_GAMES: [(&str, &str); 5] = [
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
    (
        "Animal Race",
        "https://raw.githubusercontent.com/kripod/chip8-roms/refs/heads/master/games/Animal%20Race%20%5BBrian%20Astle%5D.ch8",
    ),
];

fn online_program_picker(app_state: &'_ ApplicationState) -> Element<'_, Message> {
    let fetching = app_state.fetching_data;
    scrollable(
        Column::from_iter(DEFAULT_GAMES.map(|(name, url)| {
            let btn = button(name)
                .padding(8)
                .style(button::subtle)
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
