use iced::Element;

use crate::main_pane_view::main_pane;
use crate::{ApplicationState, Message};

pub fn application_view(app_state: &'_ ApplicationState) -> Element<'_, Message> {
    main_pane(app_state)
}
