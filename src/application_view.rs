// TODO:
// use iced::keyboard;
use iced::widget::pane_grid::{self, PaneGrid};
use iced::widget::{button, column, container, pick_list, responsive, row, text};
use iced::{Center, Color, Element, Fill, Length};

use crate::file_picker_view::file_picker;
use crate::{AVAILABLE_VIEWS, InterpreterPaneViewKind};
use crate::{
    ApplicationState, Message, controls_view::controls,
    interpreter_screen_view::interpreter_screen, keypad_view::keypad, metadata_view::metadata,
};

const PANE_HEADER_TEXT_COLOR: Color = Color::from_rgba(
    0xaa as f32 / 255.0,
    0x00 as f32 / 255.0,
    0x00 as f32 / 255.0,
    0x00 as f32 / 255.0,
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
            "Φ",
            text(pane.id.to_string()).color(PANE_HEADER_TEXT_COLOR),
        ]
        .spacing(5);

        let title_bar = pane_grid::TitleBar::new(title)
            .controls(pane_grid::Controls::dynamic(
                view_full_panel_controls(app_state, id, pane.is_pinned, is_maximized),
                view_partial_panel_controls(app_state, id, pane.is_pinned, is_maximized),
            ))
            .padding(5)
            .style(if is_focused {
                style::title_bar_focused
            } else {
                style::title_bar_active
            });
        pane_grid::Content::new(responsive(move |_| {
            column![view_interpreter_pane(app_state, pane.id)]
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

fn view_interpreter_pane(app_state: &'_ ApplicationState, id: usize) -> Element<'_, Message> {
    let selected = app_state.pane_purposes.get(&id);

    let comp: Option<Element<'_, Message>> = selected.map(|x| match x {
        InterpreterPaneViewKind::ScreenView => interpreter_screen(app_state).into(),
        InterpreterPaneViewKind::ControllerView => controls(app_state),
        InterpreterPaneViewKind::MetadataView => metadata(app_state).into(),
        InterpreterPaneViewKind::Keypad => keypad(app_state),
        InterpreterPaneViewKind::ProgramPickerView => file_picker(app_state),
    });

    let list = pick_list(AVAILABLE_VIEWS, selected, move |x| {
        Message::PaneSetActiveView(x, id)
    });
    let content = column![list, comp,].spacing(10);

    container(content).padding(5).into()
}

fn view_partial_panel_controls(
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
            "Split Horizontally <|>",
            Message::PaneSplit(pane_grid::Axis::Horizontal, pane),
        ),
        button(
            "Split Vertically <-->",
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
            text_color: Some(palette.primary),
            background: Some(palette.background.into()),
            border: Border {
                width: 0.5,
                color: palette.text,
                ..Border::default()
            },
            ..Default::default()
        }
    }

    pub fn title_bar_focused(theme: &Theme) -> container::Style {
        let palette = theme.palette();

        container::Style {
            text_color: Some(palette.primary),
            background: Some(palette.background.into()),
            border: Border {
                width: 1.0,
                color: palette.text,
                ..Border::default()
            },
            ..Default::default()
        }
    }

    pub fn pane_active(theme: &Theme) -> container::Style {
        let palette = theme.palette();

        container::Style {
            background: Some(palette.background.into()),
            border: Border {
                width: 0.5,
                color: palette.text,
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
                width: 1.0,
                color: palette.text,
                ..Border::default()
            },
            ..Default::default()
        }
    }
}
