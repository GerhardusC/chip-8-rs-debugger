use std::error::Error;

use chip_eight_debugger::{
    ApplicationState, application_subs, application_update, application_view,
};

fn main() -> Result<(), Box<dyn Error>> {
    iced::application(
        ApplicationState::default,
        application_update,
        application_view,
    )
    .theme(ApplicationState::theme)
    .subscription(application_subs)
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
