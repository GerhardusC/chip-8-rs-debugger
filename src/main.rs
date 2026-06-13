use std::{cell::RefCell, rc::Rc};

use chip_eight::{Draw, Emulator, ReadInputState};
use iced::{
    Color, Renderer, Theme, mouse,
    widget::{Canvas, Column, button, canvas, column, text},
};

fn main() {
    iced::run(update, view).unwrap();
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
}

fn update(value: &mut ApplicationState, message: Message) {
    value.emulator.0.borrow_mut().next();
    match message {
        Message::Increment => value.value += 1,
    }
}

fn view(value: &'_ ApplicationState) -> Column<'_, Message> {
    let x: Canvas<_, Message> = canvas(ApplicationState::default());
    column![
        text(value.value),
        button("+").on_press(Message::Increment),
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
    value: u32,
    emulator: EmulatorWrapper,
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
