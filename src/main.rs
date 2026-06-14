use std::{cell::RefCell, error::Error, rc::Rc};

use chip_eight::{Draw, Emulator, EmulatorState, ReadInputState};
use iced::{
    Length, Subscription,
    time::{self, Duration},
    widget::{Column, Image, button, column, image::Handle, text},
};

fn main() -> Result<(), Box<dyn Error>> {
    iced::application(ApplicationState::default, update, view)
        .subscription(ApplicationState::subscription)
        .run()?;
    Ok(())
}

#[derive(Debug, Clone)]
enum Message {
    NextInstruction,
    TempLoadProgram,
    ToggleRunning,
    Tick,
}

fn update(value: &mut ApplicationState, message: Message) {
    match message {
        Message::NextInstruction => {
            let emulator_state = value.emulator.0.borrow_mut().next();
            if let Some(emulator_state) = emulator_state {
                value.emulator_state = emulator_state;
            }
        }
        Message::TempLoadProgram => {
            let program = std::fs::read("/home/gerhardus/Downloads/Pong (1 player).ch8");
            if let Ok(program) = program {
                let _ = value.emulator.0.borrow_mut().reset(program);
            }
        }
        Message::Tick => {
            let emulator_state = value.emulator.0.borrow_mut().next();
            if let Some(emulator_state) = emulator_state {
                value.emulator_state = emulator_state;
            }
        }
        Message::ToggleRunning => {
            value.is_running = !value.is_running;
        }
    }
}

fn view(value: &'_ ApplicationState) -> Column<'_, Message> {
    let EmulatorState {
        stack,
        variable_registers,
        index_register,
        program_counter,
        last_instruction,
        screen_buffer,
        width,
        height,
        ..
    } = &value.emulator_state;

    let pixels = screen_buffer
        .iter()
        .flat_map(|px| {
            if *px > 0 {
                [0xFF, 0xFF, 0xFF, 0xFF]
            } else {
                [0x0, 0x0, 0x0, 0xFF]
            }
        })
        .collect::<Vec<u8>>();

    let image_handle =
        iced::widget::image::Handle::from_rgba(*width as u32, *height as u32, pixels);
    let image_component: Image<Handle> = iced::widget::image(image_handle)
        .width(Length::Fixed(512.0))
        .height(Length::Fixed(256.0));

    let stack = format!("STACK:              {:?}", stack);
    let variable_registers = format!("VARIABLE REGISTERS: {:?}", variable_registers);
    let index_register = format!("INDEX REGISTER:     {:?}", index_register);
    let program_counter = format!("PROGRAM_COUNTER:    {:?}", program_counter);
    let last_instruction = format!("LAST_INSTRUCTION:   {:?}", last_instruction);
    column![
        text(stack),
        text(variable_registers),
        text(index_register),
        text(program_counter),
        text(last_instruction),
        button("Next").on_press(Message::NextInstruction),
        button("Run/Stop").on_press(Message::ToggleRunning),
        button("Load Program").on_press(Message::TempLoadProgram),
        image_component,
    ]
}

#[derive(Debug, Clone)]
struct EmulatorWrapper(Rc<RefCell<Emulator<Drawer, InputReader>>>);

impl Default for EmulatorWrapper {
    fn default() -> Self {
        let mut emulator =
            Emulator::init(vec![0, 0, 0], Drawer, InputReader).expect("Program not too big");

        emulator.set_max_draw_delay(Duration::from_micros(1));

        Self(Rc::new(RefCell::new(emulator)))
    }
}

#[derive(Default)]
struct ApplicationState {
    emulator: EmulatorWrapper,
    emulator_state: EmulatorState,
    is_running: bool,
}

impl ApplicationState {
    fn subscription(&self) -> Subscription<Message> {
        if self.is_running {
            time::every(Duration::from_millis(4)).map(|_| Message::Tick)
        } else {
            Subscription::none()
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Drawer;

impl Draw for Drawer {}

#[derive(Debug, Clone, Default)]
struct InputReader;
impl ReadInputState for InputReader {
    fn read_keys_state(&self) -> Result<[u8; 16], String> {
        Ok([0; 16])
    }
    fn reset_keys_state(&mut self) {}
}
