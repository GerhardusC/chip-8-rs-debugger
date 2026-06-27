use std::{
    cell::RefCell, collections::HashMap, fmt::Display, path::PathBuf, rc::Rc, time::Duration,
};

use chip_eight::{Draw, Emulator, EmulatorState, Instruction, ReadInputState};
use iced::{
    Theme,
    widget::pane_grid::{self, Configuration},
};

mod application_update;
mod application_view;
mod controls_update;
mod controls_view;
mod emulator_update;
mod file_picker_update;
mod file_picker_view;
mod interpreter_screen_view;
mod keyboard_update;
mod keypad_view;
mod main_pane_update;
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
                a: Box::new(Configuration::Pane(PaneState::new(0))),
                b: Box::new(Configuration::Split {
                    axis: pane_grid::Axis::Vertical,
                    ratio: 0.75,
                    a: Box::new(Configuration::Pane(PaneState::new(1))),
                    b: Box::new(Configuration::Pane(PaneState::new(2))),
                }),
            }),
            b: Box::new(Configuration::Split {
                axis: pane_grid::Axis::Horizontal,
                ratio: 0.70,
                a: Box::new(Configuration::Pane(PaneState::new(3))),
                b: Box::new(Configuration::Pane(PaneState::new(4))),
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
            metadata: Default::default(),
            breakpoint: None,
            execution_speed: 90,
            theme: Some(Theme::Nord),
            quirks_mode: SupportedQuirksModes::Chip8,
            program_source: ProgramPickerSource::Online,
            fetching_data: false,
            program_path: String::new(),
        }
    }
}

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

    pub fn theme(&self) -> Option<Theme> {
        self.theme.clone()
    }
}

#[derive(Debug, Clone)]
pub enum EmulatorKeyEvent {
    Up,
    Down,
    Toggle,
}

pub enum EmulatorKeyboardInputKind {
    UsKeyboardChar(char),
    HexKeyIndex(u8),
}

#[derive(Clone, Copy, Default)]
pub struct PaneState {
    pub id: usize,
    pub is_pinned: bool,
}

impl PaneState {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            is_pinned: false,
        }
    }
}

pub struct ApplicationState {
    pub emulator: EmulatorWrapper,
    pub emulator_state: EmulatorState,
    pub is_running: bool,
    pub panes: pane_grid::State<PaneState>,
    pub panes_created: usize,
    pub focus: Option<pane_grid::Pane>,
    pub pane_purposes: HashMap<usize, InterpreterPaneViewKind>,
    pub current_dir: Vec<PathBuf>,
    pub parent_dir: Option<PathBuf>,
    pub current_program: Vec<Instruction>,
    pub auto_scroll_pc: bool,
    pub metadata: MetaData,
    pub breakpoint: Option<usize>,
    pub execution_speed: u8,
    pub theme: Option<Theme>,
    pub quirks_mode: SupportedQuirksModes,
    // TODO: Group similar things together here
    pub program_source: ProgramPickerSource,
    pub program_path: String,
    pub fetching_data: bool,
}

pub struct MetaData {
    pub register_x: Option<usize>,
    pub register_y: Option<usize>,
    pub draw_height: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProgramPickerSource {
    Disk,
    Online,
}

impl Display for ProgramPickerSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProgramPickerSource::Disk => f.write_str("Select from Disk"),
            ProgramPickerSource::Online => f.write_str("Select from Online"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SupportedQuirksModes {
    Chip8,
    SuperChip,
}

impl Display for SupportedQuirksModes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SupportedQuirksModes::Chip8 => f.write_str("Chip-8 Quirks"),
            SupportedQuirksModes::SuperChip => f.write_str("Super Chip Quirks"),
        }
    }
}

impl Default for MetaData {
    fn default() -> Self {
        Self {
            register_x: Default::default(),
            register_y: Default::default(),
            draw_height: 5,
        }
    }
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
