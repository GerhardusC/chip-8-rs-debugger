use chip_eight::EmulatorState;
use iced::{
    Length,
    widget::{Image, image::Handle},
};

use crate::ApplicationState;

pub fn interpreter_screen(app_state: &'_ ApplicationState) -> Image<Handle> {
    let EmulatorState {
        screen_buffer,
        width,
        height,
        ..
    } = &app_state.emulator_state;

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
    iced::widget::image(image_handle)
        .width(Length::Fill)
        .height(Length::Fill)
}
