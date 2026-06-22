use std::path::PathBuf;

use chip_eight::Instruction;
use iced::{
    Task,
    widget::{
        operation::{RelativeOffset, snap_to},
        pane_grid,
    },
};

use crate::{ApplicationState, InterpreterPaneViewKind, PC_START};

#[derive(Debug, Clone)]
pub enum Message {
    // COMBINED CONTROLS
    PaneSetActiveView(InterpreterPaneViewKind, usize),

    // FILE CONTROLS
    LoadProgram(PathBuf),
    EnterDirectory(PathBuf),
    UpdateProgram(Vec<u8>),
    FsError,
    UpdateDirectoryListing(Vec<PathBuf>),

    // EMULATOR CONTROLS
    NextInstruction,
    KeyToggled(u8),
    ToggleRunning,

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

#[derive(Clone, Copy, Default)]
pub struct Pane {
    pub id: usize,
    pub is_pinned: bool,
}

impl Pane {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            is_pinned: false,
        }
    }
}

pub fn application_update(
    application_state: &mut ApplicationState,
    message: Message,
) -> Task<Message> {
    match message {
        Message::NextInstruction => {
            let emulator_state = application_state.emulator.0.borrow_mut().next();
            if let Some(emulator_state) = emulator_state {
                application_state.emulator_state = emulator_state;
            }
            let pc = application_state.emulator_state.program_counter;
            let program_len = application_state.current_program.len();
            if pc < PC_START || program_len == 0 {
                return Task::none();
            }

            // Lol
            let normalised_pc = (application_state.emulator_state.program_counter - PC_START) / 2;
            // TODO: Stick this hard coded 10 into a slider
            let position = (normalised_pc + 10) as f32 / program_len as f32;
            if application_state.emulator_state.program_counter >= PC_START {
                return snap_to(
                    "program_list",
                    RelativeOffset {
                        x: 0.0,
                        y: position,
                    },
                );
            }
        }
        Message::ToggleRunning => {
            application_state.is_running = !application_state.is_running;
        }
        Message::KeyToggled(key) => {
            if let Some(key) = application_state
                .emulator
                .0
                .borrow_mut()
                .input_provider
                .keys_state
                .get_mut(key as usize & 0xF)
            {
                *key = if *key > 0 { 0 } else { 1 };
            };
        }
        Message::LoadProgram(path_buf) => {
            return Task::perform(async { std::fs::read(path_buf) }, |x| {
                if let Ok(x) = x {
                    Message::UpdateProgram(x)
                } else {
                    Message::FsError
                }
            });
        }
        Message::UpdateProgram(program) => {
            application_state.current_program = program
                .chunks(2)
                .map(|c| {
                    if let (Some(a), Some(b)) = (c.first(), c.get(1)) {
                        ((*a as u16) << 8 | *b as u16).into()
                    } else {
                        Instruction::Unimplemented(0)
                    }
                })
                .collect();
            if let Err(e) = application_state.emulator.0.borrow_mut().reset(program) {
                eprintln!("Program too large: {e}");
            };
        }
        Message::EnterDirectory(path_buf) => {
            application_state.parent_dir = path_buf.parent().map(|x| x.to_path_buf());

            return Task::perform(
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
                        Message::FsError
                    }
                },
            );
        }
        Message::UpdateDirectoryListing(current_dir) => {
            application_state.current_dir = current_dir;
        }
        Message::PaneSplit(axis, pane) => {
            let result = application_state.panes.split(
                axis,
                pane,
                Pane::new(application_state.panes_created),
            );

            if let Some((pane, _)) = result {
                application_state.focus = Some(pane);
            }

            application_state.panes_created += 1;
        }
        Message::PaneSplitFocused(axis) => {
            if let Some(pane) = application_state.focus {
                let result = application_state.panes.split(
                    axis,
                    pane,
                    Pane::new(application_state.panes_created),
                );

                if let Some((pane, _)) = result {
                    application_state.focus = Some(pane);
                }

                application_state.panes_created += 1;
            }
        }
        Message::PaneFocusAdjacent(direction) => {
            if let Some(pane) = application_state.focus
                && let Some(adjacent) = application_state.panes.adjacent(pane, direction)
            {
                application_state.focus = Some(adjacent);
            }
        }
        Message::PaneClicked(pane) => {
            application_state.focus = Some(pane);
        }
        Message::PaneResized(pane_grid::ResizeEvent { split, ratio }) => {
            application_state.panes.resize(split, ratio);
        }
        Message::PaneDragged(pane_grid::DragEvent::Dropped { pane, target }) => {
            application_state.panes.drop(pane, target);
        }
        Message::PaneDragged(_) => {}
        Message::PaneTogglePin(pane) => {
            if let Some(Pane { is_pinned, .. }) = application_state.panes.get_mut(pane) {
                *is_pinned = !*is_pinned;
            }
        }
        Message::PaneMaximize(pane) => application_state.panes.maximize(pane),
        Message::PaneRestore => {
            application_state.panes.restore();
        }
        Message::PaneClose(pane) => {
            if let Some((_, sibling)) = application_state.panes.close(pane) {
                application_state.focus = Some(sibling);
            }
        }
        Message::PaneCloseFocused => {
            if let Some(pane) = application_state.focus
                && let Some(Pane { is_pinned, .. }) = application_state.panes.get(pane)
                && !is_pinned
                && let Some((_, sibling)) = application_state.panes.close(pane)
            {
                application_state.focus = Some(sibling);
            }
        }
        Message::PaneSetActiveView(interpreter_pane_view_kind, k) => {
            application_state
                .pane_purposes
                .insert(k, interpreter_pane_view_kind);
        }
        Message::FsError => {
            eprintln!("Failed to read file/directory");
        }
    }
    Task::none()
}
