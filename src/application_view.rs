// TODO:
// use iced::keyboard;
use iced::widget::pane_grid::{self, PaneGrid};
use iced::widget::{button, center_y, column, container, responsive, row, scrollable, text};
use iced::{Center, Color, Element, Fill, Size};

use crate::{
    ApplicationState, Message, controls_view::controls,
    interpreter_screen_view::interpreter_screen, keypad_view::keypad, metadata_view::metadata,
};

const PANE_ID_COLOR_UNFOCUSED: Color = Color::from_rgb(
    0xaa as f32 / 255.0,
    0x00 as f32 / 255.0,
    0x00 as f32 / 255.0,
);
const PANE_ID_COLOR_FOCUSED: Color = Color::from_rgb(
    0xFF as f32 / 255.0,
    0x47 as f32 / 255.0,
    0x47 as f32 / 255.0,
);

pub fn application_view(app_state: &'_ ApplicationState) -> Element<'_, Message> {
    let focus = app_state.focus;

    let pane_grid = PaneGrid::new(&app_state.panes, |id, pane, is_maximized| {
        let is_focused = focus == Some(id);

        let pin_button = button(text(if pane.is_pinned { "Unpin" } else { "Pin" }).size(14))
            .on_press(Message::PaneTogglePin(id))
            .padding(3);

        let title = row![
            pin_button,
            "Pane",
            text(pane.id.to_string()).color(if is_focused {
                PANE_ID_COLOR_FOCUSED
            } else {
                PANE_ID_COLOR_UNFOCUSED
            }),
        ]
        .spacing(5);

        let title_bar = pane_grid::TitleBar::new(title)
            .controls(pane_grid::Controls::dynamic(
                view_full_panel_controls(app_state, id, pane.is_pinned, is_maximized),
                view_partial_panel_controls(app_state, id, pane.is_pinned),
            ))
            .padding(10)
            .style(if is_focused {
                style::title_bar_focused
            } else {
                style::title_bar_active
            });
        pane_grid::Content::new(responsive(move |size| {
            column![interpreter_pane(app_state, size)]
                .spacing(10.0)
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
    .spacing(10)
    .on_click(Message::PaneClicked)
    .on_drag(Message::PaneDragged)
    .on_resize(10, Message::PaneResized);

    container(pane_grid).padding(10).into()
}

fn interpreter_pane(app_state: &'_ ApplicationState, size: Size) -> Element<'_, Message> {
    let content = column![
        text!("{}x{}", size.width, size.height).size(24),
        metadata(app_state),
        controls(app_state),
        interpreter_screen(app_state),
        keypad(app_state),
    ]
    .spacing(10)
    .align_x(Center);

    center_y(scrollable(content)).padding(5).into()
}

fn view_partial_panel_controls(
    app_state: &'_ ApplicationState,
    pane: pane_grid::Pane,
    is_pinned: bool,
) -> Element<'_, Message> {
    let button = |label, message| {
        button(text(label).width(Fill).align_x(Center).size(16))
            .width(Fill)
            .padding(8)
            .on_press(message)
    };

    let pane_controls = row![
        button("<|>", Message::PaneSplit(pane_grid::Axis::Horizontal, pane),),
        button("<-->", Message::PaneSplit(pane_grid::Axis::Vertical, pane),),
        if app_state.panes.len() > 1 && !is_pinned {
            Some(button("X", Message::PaneClose(pane)).style(button::danger))
        } else {
            None
        }
    ];
    row![
        // close,
        pane_controls,
    ]
    .into()
}

fn view_full_panel_controls(
    app_state: &'_ ApplicationState,
    pane: pane_grid::Pane,
    is_pinned: bool,
    is_maximized: bool,
) -> Element<'_, Message> {
    let maximize = if app_state.panes.len() > 1 {
        let (content, message) = if is_maximized {
            ("Restore", Message::PaneRestore)
        } else {
            ("Maximize", Message::PaneMaximize(pane))
        };

        Some(
            button(text(content).size(14))
                .style(button::secondary)
                .padding(3)
                .on_press(message),
        )
    } else {
        None
    };

    let button = |label, message| {
        button(text(label).width(Fill).align_x(Center).size(16))
            .width(Fill)
            .padding(8)
            .on_press(message)
    };

    let pane_controls = row![
        button(
            "Split Horizontally <-->",
            Message::PaneSplit(pane_grid::Axis::Horizontal, pane),
        ),
        button(
            "Split Vertically <|>",
            Message::PaneSplit(pane_grid::Axis::Vertical, pane),
        ),
        if app_state.panes.len() > 1 && !is_pinned {
            Some(button("Close", Message::PaneClose(pane)).style(button::danger))
        } else {
            None
        }
    ];

    row![maximize, pane_controls].spacing(5).into()
}

mod style {
    use iced::widget::container;
    use iced::{Border, Theme};

    pub fn title_bar_active(theme: &Theme) -> container::Style {
        let palette = theme.palette();

        container::Style {
            text_color: Some(palette.background),
            background: Some(palette.background.into()),
            ..Default::default()
        }
    }

    pub fn title_bar_focused(theme: &Theme) -> container::Style {
        let palette = theme.palette();

        container::Style {
            text_color: Some(palette.primary),
            background: Some(palette.primary.into()),
            ..Default::default()
        }
    }

    pub fn pane_active(theme: &Theme) -> container::Style {
        let palette = theme.palette();

        container::Style {
            background: Some(palette.background.into()),
            border: Border {
                width: 2.0,
                color: palette.background,
                ..Border::default()
            },
            ..Default::default()
        }
    }

    pub fn pane_focused(theme: &Theme) -> container::Style {
        let palette = theme.palette();

        container::Style {
            background: Some(palette.background.into()),
            border: Border {
                width: 2.0,
                color: palette.primary,
                ..Border::default()
            },
            ..Default::default()
        }
    }
}
