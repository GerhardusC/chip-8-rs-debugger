use iced::{
    Element,
    widget::{Column, Row, Space, column, container, row, text},
};

use crate::{ApplicationState, Message, MetaData};

pub fn metadata(app_state: &'_ ApplicationState) -> Column<'_, Message> {
    column![
        variable_registers_view(app_state),
        index_register_view(app_state),
    ]
    .spacing(5)
}

fn index_register_view(app_state: &'_ ApplicationState) -> Row<'_, Message> {
    let index_lookahead = app_state.metadata.draw_height;
    let memory_pointed_to = Column::from_iter((0..index_lookahead).map(|i| {
        let y = app_state
            .emulator_state
            .memory
            .get(app_state.emulator_state.index_register + i as usize)
            .copied()
            .unwrap_or(0_u8);
        let current_byte = format!("{:08b}", y);
        let current_byte = current_byte.chars().map(|c| {
            container(Space::new().width(18.0).height(18.0))
                .style(if c == '0' {
                    container::bordered_box
                } else {
                    container::secondary
                })
                .into()
        });
        Row::from_iter(current_byte).into()
    }));
    row![
        memory_pointed_to,
        column![
            container(text("Index Register:").style(text::secondary)),
            container(text(app_state.emulator_state.index_register))
                .style(container::bordered_box)
                .padding(10)
        ]
        .spacing(5)
        .padding(5),
        stack_view(app_state),
    ]
    .spacing(5)
}

fn stack_view(app_state: &'_ ApplicationState) -> Element<'_, Message> {
    let heading = container(text("Stack:").style(text::secondary)).padding(5);

    column![
        heading,
        Row::from_iter(app_state.emulator_state.stack.iter().map(|x| {
            container(text(x.to_string()))
                .style(container::bordered_box)
                .padding(10)
                .into()
        }))
        .spacing(5),
    ]
    .into()
}

fn variable_registers_view(app_state: &'_ ApplicationState) -> Column<'_, Message> {
    let heading = container(text("Variable registers:").style(text::secondary)).padding(5);
    let key = row![
        text("vX").style(text::primary),
        text(" | "),
        text("vY").style(text::success),
    ]
    .spacing(5);

    let header = row![heading, key,].spacing(5);

    let MetaData {
        register_x: x,
        register_y: y,
        ..
    } = app_state.metadata;

    column![
        header,
        Row::from_iter(
            app_state
                .emulator_state
                .variable_registers
                .iter()
                .enumerate()
                .map(|(i, v)| {
                    container(text(v.to_string()))
                        .style(if x == Some(i) {
                            container::primary
                        } else if y == Some(i) {
                            container::success
                        } else {
                            container::bordered_box
                        })
                        .padding(10)
                        .into()
                })
        )
        .spacing(5)
    ]
}
