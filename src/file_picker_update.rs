use std::path::PathBuf;

use iced::Task;
use surf::Url;

use crate::{ApplicationState, Message, ProgramPickerSource};

pub fn set_program_picker_source(
    application_state: &mut ApplicationState,
    source: ProgramPickerSource,
) -> Task<Message> {
    application_state.file_picker_related_data.program_source = source;
    Task::none()
}

pub fn update_program_path(
    application_state: &mut ApplicationState,
    path: String,
) -> Task<Message> {
    application_state.file_picker_related_data.program_path = path;
    Task::none()
}

pub fn load_program_from_disk(
    _application_state: &mut ApplicationState,
    path_buf: PathBuf,
) -> Task<Message> {
    Task::perform(async { std::fs::read(path_buf) }, |x| {
        if let Ok(x) = x {
            Message::UpdateProgram(x)
        } else {
            Message::ProgramFetchError
        }
    })
}

pub fn load_program_from_online(
    application_state: &mut ApplicationState,
    url: String,
) -> Task<Message> {
    if Url::parse(&url).is_err() {
        return Task::none();
    }
    application_state.file_picker_related_data.fetching_data = true;
    Task::perform(
        async {
            let res = surf::get(url).await;
            if let Ok(mut res) = res
                && let Ok(res) = res.body_bytes().await
            {
                Some(res)
            } else {
                None
            }
        },
        |program| {
            if let Some(program) = program {
                Message::UpdateProgram(program)
            } else {
                Message::ProgramFetchError
            }
        },
    )
}

pub fn enter_directory(
    application_state: &mut ApplicationState,
    path_buf: PathBuf,
) -> Task<Message> {
    application_state.file_picker_related_data.parent_dir =
        path_buf.parent().map(|x| x.to_path_buf());

    Task::perform(
        async {
            std::fs::read_dir(path_buf).map(|dir| {
                dir.flat_map(|entry| {
                    if let Ok(entry) = entry {
                        Some(entry.path())
                    } else {
                        None
                    }
                })
                .collect::<Vec<PathBuf>>()
            })
        },
        |x| {
            if let Ok(x) = x {
                Message::UpdateDirectoryListing(x)
            } else {
                Message::ProgramFetchError
            }
        },
    )
}

pub fn update_directory_listing(
    application_state: &mut ApplicationState,
    current_dir: Vec<PathBuf>,
) -> Task<Message> {
    application_state.file_picker_related_data.current_dir = current_dir;
    Task::none()
}

pub fn program_fetch_error(application_state: &mut ApplicationState) -> Task<Message> {
    application_state.file_picker_related_data.fetching_data = false;
    eprintln!("Failed to read file/directory");
    Task::none()
}
