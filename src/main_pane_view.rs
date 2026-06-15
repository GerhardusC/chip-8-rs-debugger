use iced::widget::pane_grid::{self, PaneGrid};
use iced::widget::{button, column, container, responsive, row, text};
use iced::{Center, Color, Element, Fill, Length};

use crate::interpreter_pane_view::interpreter_pane;
use crate::style;
use crate::{ApplicationState, Message};

const PANE_HEADER_TEXT_COLOR: Color = Color::from_rgba(
    0xaa as f32 / 255.0,
    0x00 as f32 / 255.0,
    0x00 as f32 / 255.0,
    0x00 as f32 / 255.0,
);

pub fn main_pane(app_state: &'_ ApplicationState) -> Element<'_, Message> {
    let focus = app_state.focus;

    let pane_grid = PaneGrid::new(&app_state.panes, |id, pane, is_maximized| {
        let is_focused = focus == Some(id);

        let pin_button = button(text(if pane.is_pinned { "Unpin" } else { "Pin" }).size(14))
            .on_press(Message::PaneTogglePin(id))
            .padding(3);

        let title = row![
            pin_button,
            "Φ",
            text(pane.id.to_string()).color(PANE_HEADER_TEXT_COLOR),
        ]
        .spacing(5);

        let title_bar = pane_grid::TitleBar::new(title)
            .controls(pane_grid::Controls::new(view_panel_controls(
                app_state,
                id,
                pane.is_pinned,
                is_maximized,
            )))
            .padding(5)
            .style(if is_focused {
                style::title_bar_focused
            } else {
                style::title_bar_active
            });
        pane_grid::Content::new(responsive(move |_| {
            column![interpreter_pane(app_state, pane.id)]
                .spacing(5.0)
                .into()
        }))
        .title_bar(title_bar)
        .style(if is_focused {
            style::pane_focused
        } else {
            style::pane_active
        })
    })
    .width(Fill)
    .height(Fill)
    .spacing(5)
    .on_click(Message::PaneClicked)
    .on_drag(Message::PaneDragged)
    .on_resize(5, Message::PaneResized);

    container(pane_grid).padding(10).into()
}

fn view_panel_controls(
    app_state: &'_ ApplicationState,
    pane: pane_grid::Pane,
    is_pinned: bool,
    is_maximized: bool,
) -> Element<'_, Message> {
    let button = |label, message| {
        button(text(label).width(Fill).align_x(Center).size(16))
            .width(Fill)
            .padding(8)
            .on_press(message)
    };

    let maximize = if app_state.panes.len() > 1 {
        let (content, message) = if is_maximized {
            ("🗕", Message::PaneRestore)
        } else {
            ("🗖", Message::PaneMaximize(pane))
        };

        Some(
            button(content, message)
                .style(button::secondary)
                .width(Length::Fixed(40.0)),
        )
    } else {
        None
    };
    let pin_button = if app_state.panes.len() > 1 && !is_pinned {
        Some(
            button("x", Message::PaneClose(pane))
                .style(button::danger)
                .width(Length::Fixed(40.0)),
        )
    } else {
        None
    };

    row![
        button("--", Message::PaneSplit(pane_grid::Axis::Horizontal, pane),)
            .width(Length::Fixed(40.0)),
        button("|", Message::PaneSplit(pane_grid::Axis::Vertical, pane),)
            .width(Length::Fixed(40.0)),
        maximize,
        pin_button,
    ]
    .spacing(10)
    .into()
}
