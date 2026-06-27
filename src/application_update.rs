use std::path::PathBuf;

use iced::{Event, Task, Theme, widget::pane_grid};

use crate::{
    ApplicationState, EmulatorKeyEvent, EmulatorKeyboardInputKind, InterpreterPaneViewKind,
    ProgramPickerSource, SupportedQuirksModes,
    controls_update::{toggle_auto_scroll_to_pc, toggle_breakpoint},
    emulator_update::{
        next_instruction, respond_to_key_event, set_execution_speed, toggle_running,
        update_program, update_quirks_mode,
    },
    file_picker_update::{
        enter_directory, load_program_from_disk, load_program_from_online, program_fetch_error,
        set_program_picker_source, update_directory_listing, update_program_path,
    },
    keyboard_update::respond_to_user_event,
    main_pane_update::{
        pane_clicked, pane_close, pane_close_focussed, pane_dragged, pane_focus_adjacent,
        pane_maximize, pane_resized, pane_restore, pane_set_active_view, pane_split,
        pane_split_focussed, pane_toggle_pin,
    },
};

#[derive(Debug, Clone)]
pub enum Message {
    // COMBINED CONTROLS
    PaneSetActiveView(InterpreterPaneViewKind, usize),

    // FILE CONTROLS
    LoadProgram(PathBuf),
    EnterDirectory(PathBuf),
    UpdateProgram(Vec<u8>),
    ProgramFetchError,
    UpdateDirectoryListing(Vec<PathBuf>),
    LoadProgramFromOnline(String),

    // EMULATOR CONTROLS
    NextInstruction,
    EmulatorKey(u8),
    ToggleRunning,
    UpdateQuirksMode(SupportedQuirksModes),

    // AUX
    ToggleAutoScrollPc,
    ToggleBreakpoint(usize),
    SetExecutionSpeed(u8),
    ThemeSelected(Theme),
    SetProgramPickerSource(ProgramPickerSource),
    UpdateProgramPath(String),

    // KEYBOARD
    UserEvent(Event),

    // PANE CONTROLS
    PaneSplit(pane_grid::Axis, pane_grid::Pane),
    PaneSplitFocused(pane_grid::Axis),
    PaneFocusAdjacent(pane_grid::Direction),
    PaneClicked(pane_grid::Pane),
    PaneDragged(pane_grid::DragEvent),
    PaneResized(pane_grid::ResizeEvent),
    PaneTogglePin(pane_grid::Pane),
    PaneMaximize(pane_grid::Pane),
    PaneRestore,
    PaneClose(pane_grid::Pane),
    PaneCloseFocused,
}

pub fn application_update(
    application_state: &mut ApplicationState,
    message: Message,
) -> Task<Message> {
    match message {
        Message::NextInstruction => next_instruction(application_state),
        Message::UpdateQuirksMode(new_mode) => update_quirks_mode(application_state, new_mode),
        Message::ToggleBreakpoint(bp) => toggle_breakpoint(application_state, bp),
        Message::SetExecutionSpeed(new_speed) => set_execution_speed(application_state, new_speed),
        Message::ToggleAutoScrollPc => toggle_auto_scroll_to_pc(application_state),
        Message::ToggleRunning => toggle_running(application_state),
        Message::EmulatorKey(key) => respond_to_key_event(
            application_state,
            EmulatorKeyEvent::Toggle,
            EmulatorKeyboardInputKind::HexKeyIndex(key),
        ),
        Message::UserEvent(event) => respond_to_user_event(application_state, event),
        Message::SetProgramPickerSource(source) => {
            set_program_picker_source(application_state, source)
        }
        Message::UpdateProgramPath(path) => update_program_path(application_state, path),
        Message::LoadProgram(path_buf) => load_program_from_disk(application_state, path_buf),
        Message::LoadProgramFromOnline(url) => load_program_from_online(application_state, url),
        Message::UpdateProgram(program) => update_program(application_state, program),
        Message::EnterDirectory(path_buf) => enter_directory(application_state, path_buf),
        Message::UpdateDirectoryListing(current_dir) => {
            update_directory_listing(application_state, current_dir)
        }
        Message::ProgramFetchError => program_fetch_error(application_state),
        Message::ThemeSelected(theme) => {
            application_state.theme = Some(theme);
            Task::none()
        }
        Message::PaneSplit(axis, pane) => pane_split(application_state, axis, pane),
        Message::PaneSplitFocused(axis) => pane_split_focussed(application_state, axis),
        Message::PaneFocusAdjacent(direction) => pane_focus_adjacent(application_state, direction),
        Message::PaneClicked(pane) => pane_clicked(application_state, pane),
        Message::PaneResized(pane_grid::ResizeEvent { split, ratio }) => {
            pane_resized(application_state, split, ratio)
        }
        Message::PaneDragged(pane_grid::DragEvent::Dropped { pane, target }) => {
            pane_dragged(application_state, pane, target)
        }
        Message::PaneTogglePin(pane) => pane_toggle_pin(application_state, pane),
        Message::PaneMaximize(pane) => pane_maximize(application_state, pane),
        Message::PaneRestore => pane_restore(application_state),
        Message::PaneClose(pane) => pane_close(application_state, pane),
        Message::PaneCloseFocused => pane_close_focussed(application_state),
        Message::PaneSetActiveView(interpreter_pane_view_kind, k) => {
            pane_set_active_view(application_state, interpreter_pane_view_kind, k)
        }
        Message::PaneDragged(_) => Task::none(),
    }
}
