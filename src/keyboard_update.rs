use iced::{
    Event, Task,
    keyboard::{self, key},
};

use crate::{
    ApplicationState, EmulatorKeyEvent, EmulatorKeyboardInputKind, Message,
    emulator_update::respond_to_key_event,
};

pub fn respond_to_user_event(
    application_state: &mut ApplicationState,
    event: Event,
) -> Task<Message> {
    match event {
        iced::Event::Keyboard(keyboard::Event::KeyPressed { key: user_key, .. }) => {
            match user_key {
                keyboard::Key::Named(key::Named::Space) => {
                    application_state.is_running = !application_state.is_running;
                }
                keyboard::Key::Character(c) => {
                    if let Some(c) = c.chars().next() {
                        return respond_to_key_event(
                            application_state,
                            EmulatorKeyEvent::Down,
                            EmulatorKeyboardInputKind::UsKeyboardChar(c),
                        );
                    };
                }
                _ => (),
            }
        }
        iced::Event::Keyboard(keyboard::Event::KeyReleased {
            key: keyboard::Key::Character(c),
            ..
        }) => {
            if let Some(c) = c.chars().next() {
                return respond_to_key_event(
                    application_state,
                    EmulatorKeyEvent::Up,
                    EmulatorKeyboardInputKind::UsKeyboardChar(c),
                );
            };
        }
        _ => (),
    };
    Task::none()
}
