use iced::{
    Task,
    widget::operation::{RelativeOffset, snap_to},
};

use crate::{ApplicationState, Message, PC_START};

const SCROLL_OFFSET: usize = 10;

pub fn toggle_breakpoint(application_state: &mut ApplicationState, bp: usize) -> Task<Message> {
    if application_state.breakpoint == Some(bp) {
        application_state.breakpoint = None;
    } else {
        application_state.breakpoint = Some(bp)
    }
    Task::none()
}

pub fn scroll_to_pc(application_state: &mut ApplicationState) -> Task<Message> {
    if !application_state.auto_scroll_pc {
        return Task::none();
    };
    // Avoiding divide by zero or subtracting with overflow
    if application_state.current_program.len() <= SCROLL_OFFSET {
        return Task::none();
    }
    let normalised_pc = application_state.get_normalised_pc().unwrap_or(0);
    let position =
        normalised_pc as f32 / (application_state.current_program.len() - SCROLL_OFFSET) as f32;
    if application_state.emulator_state.program_counter >= PC_START {
        snap_to(
            "program_list",
            RelativeOffset {
                x: 0.0,
                y: position,
            },
        )
    } else {
        Task::none()
    }
}
pub fn toggle_auto_scroll_to_pc(application_state: &mut ApplicationState) -> Task<Message> {
    application_state.auto_scroll_pc = !application_state.auto_scroll_pc;
    scroll_to_pc(application_state)
}
