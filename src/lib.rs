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
            emulator_related_data: EmulatorRelatedData {
                emulator: Default::default(),
                emulator_state: EmulatorState {
                    program_counter: PC_START,
                    ..Default::default()
                },
                is_running: Default::default(),
                current_program: vec![],
                quirks_mode: SupportedQuirksModes::Chip8,
                execution_speed: 90,
            },
            pane_related_data: PaneRelatedData {
                panes,
                panes_created: num_available_views,
                focus: None,
                pane_purposes,
            },
            file_picker_related_data: FilePickerRelatedData {
                current_dir: current_dir.unwrap_or(vec![]),
                current_search_term: String::new(),
                parent_dir: here.parent().map(|p| p.to_path_buf()),
                program_source: ProgramPickerSource::Online,
                fetching_data: false,
                program_path: String::new(),
            },
            metadata_related_data: Default::default(),
            control_related_data: ControlRelatedData {
                breakpoint: None,
                auto_scroll_pc: true,
            },
            theme: Some(Theme::Nord),
        }
    }
}

impl ApplicationState {
    /// Convert program counter value to index in current program.
    /// Returns none if PC in emulator is out of range
    pub fn get_normalised_pc(&self) -> Option<usize> {
        if self.emulator_related_data.emulator_state.program_counter < PC_START
            || self.emulator_related_data.current_program.is_empty()
        {
            return None;
        }
        let normalised_pc =
            (self.emulator_related_data.emulator_state.program_counter - PC_START) >> 1;
        Some(normalised_pc)
    }

    pub fn get_instruction_under_pc(&self) -> Option<Instruction> {
        self.get_normalised_pc()
            .and_then(|x| self.emulator_related_data.current_program.get(x))
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

pub struct EmulatorRelatedData {
    pub emulator: EmulatorWrapper,
    pub emulator_state: EmulatorState,
    pub is_running: bool,
    pub current_program: Vec<Instruction>,
    pub quirks_mode: SupportedQuirksModes,
    pub execution_speed: u8,
}
pub struct PaneRelatedData {
    pub panes: pane_grid::State<PaneState>,
    pub panes_created: usize,
    pub focus: Option<pane_grid::Pane>,
    pub pane_purposes: HashMap<usize, InterpreterPaneViewKind>,
}

pub struct ControlRelatedData {
    pub auto_scroll_pc: bool,
    pub breakpoint: Option<usize>,
}

pub struct FilePickerRelatedData {
    pub program_source: ProgramPickerSource,
    pub program_path: String,
    pub current_search_term: String,
    pub current_dir: Vec<PathBuf>,
    pub parent_dir: Option<PathBuf>,
    pub fetching_data: bool,
}

pub struct MetadataRelatedData {
    pub register_x: Option<usize>,
    pub register_y: Option<usize>,
    pub draw_height: u8,
}

pub struct ApplicationState {
    pub emulator_related_data: EmulatorRelatedData,
    pub pane_related_data: PaneRelatedData,
    pub control_related_data: ControlRelatedData,
    pub file_picker_related_data: FilePickerRelatedData,
    pub metadata_related_data: MetadataRelatedData,
    pub theme: Option<Theme>,
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

impl Default for MetadataRelatedData {
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
