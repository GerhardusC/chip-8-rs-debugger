use iced::widget::{Column, column};

use crate::{
    ApplicationState, Message, controls_view::controls,
    interpreter_screen_view::interpreter_screen, keypad_view::keypad, metadata_view::metadata,
};

pub fn application_view(app_state: &'_ ApplicationState) -> Column<'_, Message> {
    column![
        metadata(app_state),
        controls(app_state),
        interpreter_screen(app_state),
        keypad(app_state)
    ]
    .spacing(10.0)
}
