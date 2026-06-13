use std::{cell::RefCell, error::Error, rc::Rc};

use chip_eight::{Draw, Emulator, EmulatorState, ReadInputState};
use iced::{
    Color, Renderer, Subscription, Theme, mouse,
    time::{self, Duration},
    widget::{Canvas, Column, button, canvas, column, text},
};

fn main() -> Result<(), Box<dyn Error>> {
    iced::application(|| ApplicationState::default(), update, view)
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
    let x: Canvas<_, Message> = canvas(ApplicationState::default());
    let EmulatorState {
        stack,
        variable_registers,
        index_register,
        program_counter,
        last_instruction,
        ..
    } = &value.emulator_state;

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
        x
    ]
}

#[derive(Debug, Clone)]
struct EmulatorWrapper(Rc<RefCell<Emulator<Drawer, InputReader>>>);

impl Default for EmulatorWrapper {
    fn default() -> Self {
        Self(Rc::new(RefCell::new(
            Emulator::init(
                vec![0, 0, 0],
                Drawer {
                    screen_buffer: vec![0; 64 * 32],
                    width: 64,
                    height: 32,
                },
                InputReader,
            )
            .expect("Program not too big"),
        )))
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
            time::every(Duration::from_millis(10)).map(|_| Message::Tick)
        } else {
            Subscription::none()
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Drawer {
    screen_buffer: Vec<u8>,
    width: usize,
    height: usize,
}

impl Draw for Drawer {
    fn draw_buffer(&mut self, screen_buf: &[u8], screen_width: usize, screen_height: usize) {
        self.screen_buffer = screen_buf.to_vec();
        self.width = screen_width;
        self.height = screen_height;
    }

    fn clear_screen(&mut self) {
        self.screen_buffer.clear();
    }
}
#[derive(Debug, Clone, Default)]
struct InputReader;
impl ReadInputState for InputReader {
    fn read_keys_state(&self) -> Result<[u8; 16], String> {
        Ok([0; 16])
    }
    fn reset_keys_state(&mut self) {}
}

impl<Message> canvas::Program<Message> for ApplicationState {
    type State = Drawer;

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: iced::Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry<Renderer>> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        // i = y * screenwidth + x
        // x = i - ( y * screen_width)
        // y = (i - x) / screen_width
        for (i, px) in state.screen_buffer.iter().enumerate() {
            // We create a `Path` representing a simple circle
            let x = i % state.width;
            let point = canvas::Path::rectangle(
                iced::Point {
                    x: x as f32,
                    y: ((i - x) / state.width) as f32,
                },
                iced::Size {
                    width: 1.0,
                    height: 1.0,
                },
            );

            // And fill it with some color
            frame.fill(&point, if *px > 0 { Color::WHITE } else { Color::BLACK });
        }

        // Then, we produce the geometry
        vec![frame.into_geometry()]
    }
}
