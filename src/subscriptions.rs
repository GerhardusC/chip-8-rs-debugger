use std::time::Duration;

use iced::{Subscription, event, time};

use crate::{ApplicationState, Message};

pub fn application_subs(application_state: &ApplicationState) -> Subscription<Message> {
    Subscription::batch([
        interpreter_running(application_state),
        user_input_events(application_state),
    ])
}

fn interpreter_running(application_state: &ApplicationState) -> Subscription<Message> {
    if application_state.emulator_related_data.is_running
        && application_state.get_normalised_pc()
            != application_state.control_related_data.breakpoint
    {
        return time::every(Duration::from_millis(
            // The slider goes from (0;100] so have to invert
            ((101 - application_state.emulator_related_data.execution_speed) as u64) << 2,
        ))
        .map(|_| Message::NextInstruction);
    }
    Subscription::none()
}

fn user_input_events(_: &ApplicationState) -> Subscription<Message> {
    event::listen().map(Message::UserEvent)
}
