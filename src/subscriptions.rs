use std::time::Duration;

use iced::{Subscription, time};

use crate::{ApplicationState, Message};

pub fn interpreter_running(value: &ApplicationState) -> Subscription<Message> {
    if value.is_running {
        time::every(Duration::from_millis(4)).map(|_| Message::Tick)
    } else {
        Subscription::none()
    }
}
