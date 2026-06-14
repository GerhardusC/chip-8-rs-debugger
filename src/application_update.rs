use crate::ApplicationState;

#[derive(Debug, Clone)]
pub enum Message {
    NextInstruction,
    KeyToggled(u8),
    TempLoadProgram,
    ToggleRunning,
    Tick,
}

pub fn application_update(application_state: &mut ApplicationState, message: Message) {
    match message {
        Message::NextInstruction => {
            let emulator_state = application_state.emulator.0.borrow_mut().next();
            if let Some(emulator_state) = emulator_state {
                application_state.emulator_state = emulator_state;
            }
        }
        Message::TempLoadProgram => {
            let program = std::fs::read("/home/gerhardus/Downloads/Pong (1 player).ch8");
            if let Ok(program) = program {
                let _ = application_state.emulator.0.borrow_mut().reset(program);
            }
        }
        Message::Tick => {
            let emulator_state = application_state.emulator.0.borrow_mut().next();
            if let Some(emulator_state) = emulator_state {
                application_state.emulator_state = emulator_state;
            }
        }
        Message::ToggleRunning => {
            application_state.is_running = !application_state.is_running;
        }
        Message::KeyToggled(key) => {
            if let Some(key) = application_state
                .emulator
                .0
                .borrow_mut()
                .input_provider
                .keys_state
                .get_mut(key as usize & 0xF)
            {
                *key = if *key > 0 { 0 } else { 1 };
            };
        }
    }
}
