use std::{cell::RefCell, rc::Rc, time::Duration};

use chip_eight::{Draw, Emulator, EmulatorState, ReadInputState};
use iced::widget::pane_grid;

mod application_update;
mod application_view;
mod controls_view;
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

        emulator.set_max_draw_delay(Duration::from_micros(1));

        Self(Rc::new(RefCell::new(emulator)))
    }
}

impl Default for ApplicationState {
    fn default() -> Self {
        let (panes, _) = pane_grid::State::new(Pane::new(0));
        Self {
            emulator: Default::default(),
            emulator_state: Default::default(),
            is_running: Default::default(),
            panes,
            panes_created: 1,
            focus: None,
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
