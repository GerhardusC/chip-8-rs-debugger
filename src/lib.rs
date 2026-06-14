use std::{cell::RefCell, rc::Rc, time::Duration};

use chip_eight::{Draw, Emulator, EmulatorState, ReadInputState};
use iced::{Subscription, time};

mod application_update;
mod application_view;
mod keypad;

pub use application_update::*;
pub use application_view::*;

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

#[derive(Default)]
pub struct ApplicationState {
    pub emulator: EmulatorWrapper,
    pub emulator_state: EmulatorState,
    pub is_running: bool,
}

pub fn interpreter_running(value: &ApplicationState) -> Subscription<Message> {
    if value.is_running {
        time::every(Duration::from_millis(4)).map(|_| Message::Tick)
    } else {
        Subscription::none()
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
