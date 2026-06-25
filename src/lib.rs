use std::{
    cell::RefCell, collections::HashMap, fmt::Display, path::PathBuf, rc::Rc, time::Duration,
};

use chip_eight::{Draw, Emulator, EmulatorState, Instruction, ReadInputState};
use iced::{
    Task,
    widget::{
        operation::{RelativeOffset, snap_to},
        pane_grid::{self, Configuration},
    },
};

mod application_update;
mod application_view;
mod controls_view;
mod file_picker_view;
mod interpreter_pane_view;
mod interpreter_screen_view;
mod keypad_view;
mod main_pane_view;
mod metadata_view;
mod style;
mod subscriptions;

pub use application_update::*;
pub use application_view::*;
pub use subscriptions::*;

pub const PC_START: usize = 0x200;
pub const AVAILABLE_VIEWS: [InterpreterPaneViewKind; 5] = [
    InterpreterPaneViewKind::ScreenView,
    InterpreterPaneViewKind::MetadataView,
    InterpreterPaneViewKind::Keypad,
    InterpreterPaneViewKind::ControllerView,
    InterpreterPaneViewKind::ProgramPickerView,
];

#[derive(Debug, Clone)]
pub struct EmulatorWrapper(Rc<RefCell<Emulator<Drawer, Keypad>>>);

impl Default for EmulatorWrapper {
    fn default() -> Self {
        let mut emulator = Emulator::init(
            vec![],
            Drawer,
            Keypad {
                keys_state: [0; 16],
            },
        )
        .expect("Program not too big");

        emulator.set_max_draw_delay(Duration::from_micros(1));

        Self(Rc::new(RefCell::new(emulator)))
    }
}

impl Default for ApplicationState {
    fn default() -> Self {
        let config = pane_grid::Configuration::Split {
            axis: pane_grid::Axis::Vertical,
            ratio: 0.60,
            a: Box::new(Configuration::Split {
                axis: pane_grid::Axis::Horizontal,
                ratio: 0.60,
                a: Box::new(Configuration::Pane(Pane::new(0))),
                b: Box::new(Configuration::Split {
                    axis: pane_grid::Axis::Vertical,
                    ratio: 0.75,
                    a: Box::new(Configuration::Pane(Pane::new(1))),
                    b: Box::new(Configuration::Pane(Pane::new(2))),
                }),
            }),
            b: Box::new(Configuration::Split {
                axis: pane_grid::Axis::Horizontal,
                ratio: 0.70,
                a: Box::new(Configuration::Pane(Pane::new(3))),
                b: Box::new(Configuration::Pane(Pane::new(4))),
            }),
        };

        let panes = pane_grid::State::with_configuration(config);

        let num_available_views = AVAILABLE_VIEWS.len();
        let mut pane_purposes = HashMap::new();

        for (id, view_kind) in AVAILABLE_VIEWS.into_iter().enumerate() {
            pane_purposes.insert(id, view_kind);
        }

        // TODO: Make game dir configurable.
        let here = std::env::current_dir().unwrap_or(PathBuf::from("."));
        let current_dir = std::fs::read_dir(&here).map(|dir| {
            dir.flat_map(|entry| entry.map(|entry| entry.path()))
                .collect::<Vec<PathBuf>>()
        });

        Self {
            emulator: Default::default(),
            emulator_state: EmulatorState {
                program_counter: PC_START,
                ..Default::default()
            },
            is_running: Default::default(),
            panes,
            panes_created: num_available_views,
            focus: None,
            pane_purposes,
            current_dir: current_dir.unwrap_or(vec![]),
            parent_dir: here.parent().map(|p| p.to_path_buf()),
            current_program: vec![],
            auto_scroll_pc: true,
        }
    }
}

const SCROLL_OFFSET: usize = 10;

impl ApplicationState {
    /// Convert program counter value to index in current program.
    /// Returns none if PC in emulator is out of range
    pub fn get_normalised_pc(&self) -> Option<usize> {
        if self.emulator_state.program_counter < PC_START || self.current_program.is_empty() {
            return None;
        }
        let normalised_pc = (self.emulator_state.program_counter - PC_START) >> 1;
        Some(normalised_pc)
    }

    pub fn get_instruction_under_pc(&self) -> Option<Instruction> {
        self.get_normalised_pc()
            .and_then(|x| self.current_program.get(x))
            .map(|x| x.to_owned())
    }

    pub fn scroll_to_pc(&mut self) -> Task<Message> {
        if !self.auto_scroll_pc {
            return Task::none();
        };
        // Avoiding divide by zero
        if self.current_program.len() == SCROLL_OFFSET {
            return Task::none();
        }
        let normalised_pc = self.get_normalised_pc().unwrap_or(0);
        let position = normalised_pc as f32 / (self.current_program.len() - SCROLL_OFFSET) as f32;
        if self.emulator_state.program_counter >= PC_START {
            snap_to(
                "program_list",
                RelativeOffset {
                    x: 0.0,
                    y: position,
                },
            )
        } else {
            Task::none()
        }
    }
}

pub struct ApplicationState {
    pub emulator: EmulatorWrapper,
    pub emulator_state: EmulatorState,
    pub is_running: bool,
    pub panes: pane_grid::State<Pane>,
    pub panes_created: usize,
    pub focus: Option<pane_grid::Pane>,
    pub pane_purposes: HashMap<usize, InterpreterPaneViewKind>,
    pub current_dir: Vec<PathBuf>,
    pub parent_dir: Option<PathBuf>,
    pub current_program: Vec<Instruction>,
    pub auto_scroll_pc: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InterpreterPaneViewKind {
    ScreenView,
    ControllerView,
    MetadataView,
    ProgramPickerView,
    Keypad,
}

impl Display for InterpreterPaneViewKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                InterpreterPaneViewKind::ScreenView => "Screen View",
                InterpreterPaneViewKind::ControllerView => "Controller View",
                InterpreterPaneViewKind::MetadataView => "Metadata View",
                InterpreterPaneViewKind::Keypad => "Keypad",
                InterpreterPaneViewKind::ProgramPickerView => "Program Picker",
            }
        )
    }
}

#[derive(Debug, Clone, Default)]
struct Drawer;
impl Draw for Drawer {}

#[derive(Debug, Clone, Default)]
struct Keypad {
    keys_state: [u8; 16],
}

impl ReadInputState for Keypad {
    fn read_keys_state(&self) -> Result<[u8; 16], String> {
        Ok(self.keys_state)
    }
    fn reset_keys_state(&mut self) {
        self.keys_state = [0; 16];
    }
}
