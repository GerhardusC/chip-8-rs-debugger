use std::{
    cell::RefCell, collections::HashMap, fmt::Display, fs::DirEntry, path::PathBuf, rc::Rc,
    time::Duration,
};

use chip_eight::{Draw, Emulator, EmulatorState, ReadInputState};
use iced::widget::pane_grid::{self, Configuration};

mod application_update;
mod application_view;
mod controls_view;
mod file_picker_view;
mod interpreter_screen_view;
mod keypad_view;
mod metadata_view;
mod subscriptions;

pub use application_update::*;
pub use application_view::*;
pub use subscriptions::*;

#[derive(Debug, Clone)]
pub struct EmulatorWrapper(Rc<RefCell<Emulator<Drawer, Keypad>>>);

impl Default for EmulatorWrapper {
    fn default() -> Self {
        let mut emulator = Emulator::init(
            vec![0, 0, 0],
            Drawer,
            Keypad {
                keys_state: [0; 16],
            },
        )
        .expect("Program not too big");

        emulator.set_max_draw_delay(Duration::from_millis(1));

        Self(Rc::new(RefCell::new(emulator)))
    }
}

impl Default for ApplicationState {
    fn default() -> Self {
        let config = pane_grid::Configuration::Split {
            axis: pane_grid::Axis::Vertical,
            ratio: 0.70,
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
                ratio: 0.5,
                a: Box::new(Configuration::Pane(Pane::new(3))),
                b: Box::new(Configuration::Pane(Pane::new(4))),
            }),
        };

        let panes = pane_grid::State::with_configuration(config);

        let mut pane_purposes = HashMap::new();
        pane_purposes.insert(0, InterpreterPaneViewKind::ScreenView);
        pane_purposes.insert(1, InterpreterPaneViewKind::MetadataView);
        pane_purposes.insert(2, InterpreterPaneViewKind::Keypad);
        pane_purposes.insert(3, InterpreterPaneViewKind::ControllerView);
        pane_purposes.insert(4, InterpreterPaneViewKind::ProgramPickerView);

        let here = std::env::current_dir().unwrap_or(PathBuf::from("."));
        let current_dir =
            std::fs::read_dir(here).map(|dir| dir.flatten().collect::<Vec<DirEntry>>());

        Self {
            emulator: Default::default(),
            emulator_state: Default::default(),
            is_running: Default::default(),
            panes,
            panes_created: 4,
            focus: None,
            pane_purposes,
            current_dir: current_dir.unwrap_or(vec![]),
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
    pub current_dir: Vec<DirEntry>,
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
