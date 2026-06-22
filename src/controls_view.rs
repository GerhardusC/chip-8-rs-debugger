use iced::{
    Element,
    widget::{self, button, column, row, scrollable, text},
};

use crate::{ApplicationState, Message, PC_START};

pub fn controls(app_state: &'_ ApplicationState) -> Element<'_, Message> {
    let instructions_list = column(app_state.current_program.iter().enumerate().map(
        |(i, instruction)| {
            // TODO: Nice formatting for instructions
            // offset.
            let text = text(format!("{}: {:?}", (i * 2) + PC_START, instruction))
                .wrapping(text::Wrapping::None);
            let pc = app_state.emulator_state.program_counter;
            if pc >= PC_START && (pc - PC_START) / 2 == i {
                text.style(text::primary).into()
            } else {
                text.style(text::secondary).into()
            }
        },
    ));

    // TODO: Move to Id::unique()
    // TODO: Add toggle for auto scroll
    let program_list = scrollable(instructions_list).id(widget::Id::new("program_list"));

    let no_prog_warning = if app_state.current_program.is_empty() {
        Some(text("No program loaded").style(text::secondary))
    } else {
        None
    };

    let run_button = if app_state.is_running {
        button("⏸")
            .on_press(Message::ToggleRunning)
            .style(button::danger)
    } else {
        button("▶")
            .on_press(Message::ToggleRunning)
            .style(button::success)
    };

    column![
        row![
            button("Next").on_press(Message::NextInstruction),
            run_button,
            no_prog_warning
        ]
        .spacing(5.0),
        program_list
    ]
    .into()
}
