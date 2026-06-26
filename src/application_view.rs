use iced::Alignment::Center;
use iced::widget::{column, pick_list, row, space, text};
use iced::{Element, Theme};

use crate::main_pane_view::main_pane;
use crate::{ApplicationState, Message};

pub fn application_view(app_state: &'_ ApplicationState) -> Element<'_, Message> {
    let header = row![
        space::horizontal(),
        text("Select Theme").size(16).style(text::secondary),
        pick_list(Theme::ALL, app_state.theme.as_ref(), Message::ThemeSelected),
    ]
    .padding(5)
    .spacing(20)
    .align_y(Center);

    column![header, main_pane(app_state)].into()
}
