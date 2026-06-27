use std::path::PathBuf;

use chip_eight::{Instruction, QuirksFields, QuirksMode, SuperChipBehaviour};
use iced::{
    Event, Task, Theme,
    keyboard::{self, key},
    widget::pane_grid,
};
use surf::Url;

use crate::{
    ApplicationState, EmulatorKeyEvent, EmulatorKeyboardInputKind, InterpreterPaneViewKind,
    ProgramPickerSource, SupportedQuirksModes, respond_to_key_event,
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
            let instruction = application_state.get_instruction_under_pc();
            let (x, y, h) = if let Some(instruction) = instruction {
                match instruction {
                    Instruction::AddToRegister { register: x, .. }
                    | Instruction::JumpWithOffset { register_x: x, .. }
                    | Instruction::Random { register_x: x, .. }
                    | Instruction::SetGeneralRegister { register: x, .. }
                    | Instruction::SkipEqValueWithRegisterContents { register: x, .. }
                    | Instruction::SkipIfKey { register: x, .. }
                    | Instruction::SkipNotEqValueWithRegisterContents { register: x, .. }
                    | Instruction::SubCommand { register: x, .. } => (Some(x), None, None),
                    Instruction::SkipEqRegisters {
                        register_x: x,
                        register_y: y,
                    }
                    | Instruction::SkipNotEqRegisters {
                        register_x: x,
                        register_y: y,
                    }
                    | Instruction::LogicalOperator {
                        register_x: x,
                        register_y: y,
                        ..
                    } => (Some(x), Some(y), None),
                    Instruction::Draw {
                        x_register: x,
                        y_register: y,
                        height: h,
                    } => (Some(x), Some(y), Some(h)),
                    _ => (None, None, None),
                }
            } else {
                (None, None, None)
            };
            application_state.metadata.register_x = x;
            application_state.metadata.register_y = y;
            if let Some(height) = h {
                if height == 0 {
                    application_state.metadata.draw_height = 16;
                } else {
                    application_state.metadata.draw_height = height;
                }
            }

            application_state.scroll_to_pc()
        }
        Message::ToggleBreakpoint(bp) => {
            if application_state.breakpoint == Some(bp) {
                application_state.breakpoint = None;
            } else {
                application_state.breakpoint = Some(bp)
            }
            Task::none()
        }
        Message::SetExecutionSpeed(new_speed) => {
            application_state.execution_speed = new_speed;
            Task::none()
        }
        Message::ToggleAutoScrollPc => {
            application_state.auto_scroll_pc = !application_state.auto_scroll_pc;
            application_state.scroll_to_pc()
        }
        Message::ToggleRunning => {
            application_state.is_running = !application_state.is_running;
            Task::none()
        }
        Message::EmulatorKey(key) => {
            respond_to_key_event(
                application_state,
                EmulatorKeyEvent::Toggle,
                EmulatorKeyboardInputKind::HexKeyIndex(key),
            );
            Task::none()
        }
        Message::UserEvent(event) => match event {
            Event::Keyboard(event) => {
                match event {
                    keyboard::Event::KeyPressed { key: user_key, .. } => match user_key {
                        keyboard::Key::Named(key::Named::Space) => {
                            application_state.is_running = !application_state.is_running;
                        }
                        keyboard::Key::Character(c) => {
                            if let Some(c) = c.chars().next() {
                                respond_to_key_event(
                                    application_state,
                                    EmulatorKeyEvent::Down,
                                    EmulatorKeyboardInputKind::UsKeyboardChar(c),
                                );
                            };
                        }
                        _ => (),
                    },
                    keyboard::Event::KeyReleased {
                        key: keyboard::Key::Character(c),
                        ..
                    } => {
                        if let Some(c) = c.chars().next() {
                            respond_to_key_event(
                                application_state,
                                EmulatorKeyEvent::Up,
                                EmulatorKeyboardInputKind::UsKeyboardChar(c),
                            );
                        };
                    }
                    _ => (),
                };
                Task::none()
            }
            _ => Task::none(),
        },
        Message::SetProgramPickerSource(program_picker_source) => {
            application_state.program_source = program_picker_source;
            Task::none()
        }
        Message::UpdateProgramPath(path) => {
            application_state.program_path = path;
            Task::none()
        }
        Message::LoadProgram(path_buf) => Task::perform(async { std::fs::read(path_buf) }, |x| {
            if let Ok(x) = x {
                Message::UpdateProgram(x)
            } else {
                Message::ProgramFetchError
            }
        }),
        Message::LoadProgramFromOnline(url) => {
            if Url::parse(&url).is_err() {
                // TODO: Handle invalid URL
                return Task::none();
            }
            application_state.fetching_data = true;
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
        Message::UpdateProgram(program) => {
            application_state.fetching_data = false;
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

            let quirks = chip_eight::QuirksFields::from(chip_eight::QuirksMode::Chip8);
            application_state
                .emulator
                .0
                .borrow_mut()
                .with_custom_quirks(QuirksFields {
                    disp_wait: false,
                    ..quirks
                });

            application_state.scroll_to_pc()
        }
        Message::UpdateQuirksMode(new_mode) => {
            let quirks_mode = match &new_mode {
                SupportedQuirksModes::Chip8 => QuirksMode::Chip8,
                SupportedQuirksModes::SuperChip => {
                    QuirksMode::SuperChip(SuperChipBehaviour::Modern)
                }
            };
            application_state.quirks_mode = new_mode;

            let quirks = chip_eight::QuirksFields::from(quirks_mode);
            application_state
                .emulator
                .0
                .borrow_mut()
                .with_custom_quirks(QuirksFields {
                    disp_wait: false,
                    ..quirks
                });
            Task::none()
        }
        Message::EnterDirectory(path_buf) => {
            application_state.parent_dir = path_buf.parent().map(|x| x.to_path_buf());

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
        Message::UpdateDirectoryListing(current_dir) => {
            application_state.current_dir = current_dir;
            Task::none()
        }
        Message::ProgramFetchError => {
            application_state.fetching_data = false;
            eprintln!("Failed to read file/directory");
            Task::none()
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
            Task::none()
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
            Task::none()
        }
        Message::PaneFocusAdjacent(direction) => {
            if let Some(pane) = application_state.focus
                && let Some(adjacent) = application_state.panes.adjacent(pane, direction)
            {
                application_state.focus = Some(adjacent);
            }
            Task::none()
        }
        Message::PaneClicked(pane) => {
            application_state.focus = Some(pane);
            Task::none()
        }
        Message::PaneResized(pane_grid::ResizeEvent { split, ratio }) => {
            application_state.panes.resize(split, ratio);
            Task::none()
        }
        Message::PaneDragged(pane_grid::DragEvent::Dropped { pane, target }) => {
            application_state.panes.drop(pane, target);
            Task::none()
        }
        Message::PaneDragged(_) => Task::none(),
        Message::PaneTogglePin(pane) => {
            if let Some(Pane { is_pinned, .. }) = application_state.panes.get_mut(pane) {
                *is_pinned = !*is_pinned;
            }
            Task::none()
        }
        Message::PaneMaximize(pane) => {
            application_state.panes.maximize(pane);
            Task::none()
        }
        Message::PaneRestore => {
            application_state.panes.restore();
            Task::none()
        }
        Message::PaneClose(pane) => {
            if let Some((_, sibling)) = application_state.panes.close(pane) {
                application_state.focus = Some(sibling);
            }
            Task::none()
        }
        Message::PaneCloseFocused => {
            if let Some(pane) = application_state.focus
                && let Some(Pane { is_pinned, .. }) = application_state.panes.get(pane)
                && !is_pinned
                && let Some((_, sibling)) = application_state.panes.close(pane)
            {
                application_state.focus = Some(sibling);
            }
            Task::none()
        }
        Message::PaneSetActiveView(interpreter_pane_view_kind, k) => {
            application_state
                .pane_purposes
                .insert(k, interpreter_pane_view_kind);
            Task::none()
        }
        Message::ThemeSelected(theme) => {
            application_state.theme = Some(theme);
            Task::none()
        }
    }
}
