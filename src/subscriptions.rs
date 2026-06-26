use std::time::Duration;

use iced::{Subscription, time};

use crate::{ApplicationState, Message};

pub fn interpreter_running(application_state: &ApplicationState) -> Subscription<Message> {
    if application_state.is_running
        && application_state.get_normalised_pc() != application_state.breakpoint
    {
        return time::every(Duration::from_millis(
            // The slider goes from (0;100] so have to invert
            ((101 - application_state.execution_speed) as u64) << 2,
        ))
        .map(|_| Message::NextInstruction);
    }
    Subscription::none()
}
