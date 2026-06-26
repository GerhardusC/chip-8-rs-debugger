use chip_eight::EmulatorState;
use iced::{Color, Element, Length, widget::container};

use crate::{ApplicationState, Message};

// TODO: See if it helps perf migrating to canvas
pub fn interpreter_screen(app_state: &'_ ApplicationState) -> Element<'_, Message> {
    let EmulatorState {
        screen_buffer,
        width,
        height,
        ..
    } = &app_state.emulator_state;

    let (top_colour, bottom_colour) = if let Some(theme) = &app_state.theme {
        (
            theme.extended_palette().primary.base.color,
            theme.extended_palette().background.base.color,
        )
    } else {
        (
            Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            },
            Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            },
        )
    };

    let pixels = screen_buffer
        .iter()
        .flat_map(|px| {
            if *px > 0 {
                [
                    (255.0 * top_colour.r) as u8,
                    (255.0 * top_colour.g) as u8,
                    (255.0 * top_colour.b) as u8,
                    (255.0 * top_colour.a) as u8,
                ]
            } else {
                [
                    (255.0 * bottom_colour.r) as u8,
                    (255.0 * bottom_colour.g) as u8,
                    (255.0 * bottom_colour.b) as u8,
                    (255.0 * bottom_colour.a) as u8,
                ]
            }
        })
        .collect::<Vec<u8>>();

    let image_handle =
        iced::widget::image::Handle::from_rgba(*width as u32, *height as u32, pixels);
    container(
        iced::widget::image(image_handle)
            .width(Length::Fill)
            .height(Length::Fill)
            .filter_method(iced::widget::image::FilterMethod::Nearest),
    )
    .style(container::bordered_box)
    .into()
}
