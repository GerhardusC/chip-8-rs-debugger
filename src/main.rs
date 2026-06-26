use std::error::Error;

use chip_eight_debugger::{
    ApplicationState, application_update, application_view, interpreter_running,
};

fn main() -> Result<(), Box<dyn Error>> {
    iced::application(
        ApplicationState::default,
        application_update,
        application_view,
    )
    .theme(ApplicationState::theme)
    .subscription(interpreter_running)
    .window(iced::window::Settings {
        size: iced::Size {
            width: 1440.0,
            height: 900.0,
        },
        ..Default::default()
    })
    .run()?;
    Ok(())
}
