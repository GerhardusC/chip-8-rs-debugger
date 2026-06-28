use std::{
    io::{BufRead, BufReader},
    path::PathBuf,
    sync::LazyLock,
};

use iced::{
    Element, Length,
    widget::{Column, TextInput, button, column, pick_list, row, scrollable, space, text},
};
use surf::Url;

use crate::{ApplicationState, Message, ProgramPickerSource};

pub fn file_picker(app_state: &'_ ApplicationState) -> Element<'_, Message> {
    let program_source_pick_list = pick_list(
        [ProgramPickerSource::Disk, ProgramPickerSource::Online],
        Some(&app_state.file_picker_related_data.program_source),
        Message::SetProgramPickerSource,
    );

    let (program_picker, on_submit, on_input, label, online_game_search_bar) =
        match &app_state.file_picker_related_data.program_source {
            ProgramPickerSource::Disk => {
                let program_path = PathBuf::from(
                    if app_state
                        .file_picker_related_data
                        .program_path
                        .starts_with("~")
                    {
                        std::env!("HOME").to_owned()
                            + app_state
                                .file_picker_related_data
                                .program_path
                                .trim_start_matches("~")
                    } else {
                        app_state.file_picker_related_data.program_path.to_owned()
                    },
                );
                (
                    file_program_picker(app_state),
                    if program_path.is_dir() {
                        Message::EnterDirectory(program_path)
                    } else {
                        Message::LoadProgram(program_path)
                    },
                    Message::UpdateProgramPath,
                    "File or Dir",
                    None,
                )
            }
            ProgramPickerSource::Online => (
                online_program_picker(app_state),
                Message::LoadProgramFromOnline(
                    app_state.file_picker_related_data.program_path.to_owned(),
                ),
                Message::UpdateProgramPath,
                "URL",
                Some(
                    TextInput::new(
                        "Search",
                        &app_state.file_picker_related_data.current_search_term,
                    )
                    .on_input(Message::SetCurrentSearchTerm),
                ),
            ),
        };

    let submit_btn = button("⬇️");
    let input = TextInput::new(label, &app_state.file_picker_related_data.program_path);
    let (input, submit_btn) = if app_state.file_picker_related_data.fetching_data {
        (input, submit_btn)
    } else {
        (
            input.on_submit(on_submit.clone()).on_input(on_input),
            submit_btn.on_press(on_submit),
        )
    };
    let top_row = row![
        program_source_pick_list,
        online_game_search_bar,
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

    let up_a_dir_button =
        app_state
            .file_picker_related_data
            .parent_dir
            .as_ref()
            .map(|parent_dir| {
                Element::from(
                    button(
                        "..".to_owned(),
                        Message::EnterDirectory(parent_dir.to_owned()),
                    )
                    .style(button::secondary),
                )
            });
    let mut files = app_state.file_picker_related_data.current_dir.clone();

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

static FALLBACK_GAMES: [&str; 9] = [
    "https://raw.githubusercontent.com/JohnEarnest/chip8Archive/refs/heads/master/roms/rockto.ch8",
    "https://raw.githubusercontent.com/Timendus/chip8-test-suite/refs/heads/main/bin/1-chip8-logo.ch8",
    "https://raw.githubusercontent.com/Timendus/chip8-test-suite/refs/heads/main/bin/2-ibm-logo.ch8",
    "https://raw.githubusercontent.com/Timendus/chip8-test-suite/refs/heads/main/bin/3-corax%2B.ch8",
    "https://raw.githubusercontent.com/Timendus/chip8-test-suite/refs/heads/main/bin/4-flags.ch8",
    "https://raw.githubusercontent.com/Timendus/chip8-test-suite/refs/heads/main/bin/5-quirks.ch8",
    "https://raw.githubusercontent.com/Timendus/chip8-test-suite/refs/heads/main/bin/6-keypad.ch8",
    "https://raw.githubusercontent.com/Timendus/chip8-test-suite/refs/heads/main/bin/7-beep.ch8",
    "https://raw.githubusercontent.com/Timendus/chip8-test-suite/refs/heads/main/bin/8-scrolling.ch8",
];

static DEFAULT_GAMES: LazyLock<Vec<Url>> = LazyLock::new(|| {
    let Ok(f) = std::fs::File::open("games.txt") else {
        return FALLBACK_GAMES
            .iter()
            .flat_map(|game| Url::parse(game).ok())
            .collect();
    };
    let mut reader = BufReader::new(f);

    let mut paths = vec![];
    let mut current_word = String::new();
    while let Ok(bytes_read) = reader.read_line(&mut current_word)
        && bytes_read > 0
    {
        if let Ok(url) = Url::parse(&current_word) {
            paths.push(url);
        }
        current_word.clear();
    }
    if paths.is_empty() {
        paths.extend(FALLBACK_GAMES.iter().flat_map(|game| Url::parse(game).ok()));
    }
    paths
});

fn online_program_picker(app_state: &'_ ApplicationState) -> Element<'_, Message> {
    let fetching = app_state.file_picker_related_data.fetching_data;
    let games = DEFAULT_GAMES.iter();
    scrollable(
        Column::from_iter(
            games
                .filter(|url| {
                    let game_name = url
                        .path_segments()
                        .and_then(|mut split| split.next_back())
                        .and_then(|s| urlencoding::decode(s).ok().map(|s| s.to_string()))
                        .unwrap_or_else(|| url.to_string());
                    game_name.to_lowercase().contains(
                        &app_state
                            .file_picker_related_data
                            .current_search_term
                            .to_lowercase(),
                    )
                })
                .map(|url| {
                    let url = url.to_owned();
                    let game_name = url
                        .path_segments()
                        .and_then(|mut split| split.next_back())
                        .and_then(|s| urlencoding::decode(s).ok().map(|s| s.to_string()))
                        .unwrap_or_else(|| url.to_string());

                    let btn = button(text(game_name))
                        .padding(8)
                        .style(button::subtle)
                        .width(Length::Fill);
                    if fetching {
                        btn
                    } else {
                        btn.on_press(Message::LoadProgramFromOnline(url.to_string()))
                    }
                    .into()
                }),
        )
        .spacing(5),
    )
    .into()
}
