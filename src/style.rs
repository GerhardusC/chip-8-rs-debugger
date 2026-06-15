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
