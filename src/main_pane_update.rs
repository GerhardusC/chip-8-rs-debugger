// Just returning Task for all of these to make the application updage page neater, even though all
// of them are Task::none(), i.e. no messages
use iced::{
    Task,
    widget::pane_grid::{Axis, Direction, Pane, Split, Target},
};

use crate::{ApplicationState, InterpreterPaneViewKind, Message, PaneState};

pub fn pane_split(
    application_state: &mut ApplicationState,
    axis: Axis,
    pane: Pane,
) -> Task<Message> {
    let result =
        application_state
            .panes
            .split(axis, pane, PaneState::new(application_state.panes_created));

    if let Some((pane, _)) = result {
        application_state.focus = Some(pane);
    }

    application_state.panes_created += 1;
    Task::none()
}

pub fn pane_split_focussed(application_state: &mut ApplicationState, axis: Axis) -> Task<Message> {
    if let Some(pane) = application_state.focus {
        let result = application_state.panes.split(
            axis,
            pane,
            PaneState::new(application_state.panes_created),
        );

        if let Some((pane, _)) = result {
            application_state.focus = Some(pane);
        }

        application_state.panes_created += 1;
    }
    Task::none()
}

pub fn pane_focus_adjacent(
    application_state: &mut ApplicationState,
    direction: Direction,
) -> Task<Message> {
    if let Some(pane) = application_state.focus
        && let Some(adjacent) = application_state.panes.adjacent(pane, direction)
    {
        application_state.focus = Some(adjacent);
    }
    Task::none()
}

pub fn pane_clicked(application_state: &mut ApplicationState, pane: Pane) -> Task<Message> {
    application_state.focus = Some(pane);
    Task::none()
}

pub fn pane_resized(
    application_state: &mut ApplicationState,
    split: Split,
    ratio: f32,
) -> Task<Message> {
    application_state.panes.resize(split, ratio);
    Task::none()
}

pub fn pane_dragged(
    application_state: &mut ApplicationState,
    pane: Pane,
    target: Target,
) -> Task<Message> {
    application_state.panes.drop(pane, target);
    Task::none()
}

pub fn pane_toggle_pin(application_state: &mut ApplicationState, pane: Pane) -> Task<Message> {
    if let Some(PaneState { is_pinned, .. }) = application_state.panes.get_mut(pane) {
        *is_pinned = !*is_pinned;
    }
    Task::none()
}

pub fn pane_maximize(application_state: &mut ApplicationState, pane: Pane) -> Task<Message> {
    application_state.panes.maximize(pane);
    Task::none()
}

pub fn pane_restore(application_state: &mut ApplicationState) -> Task<Message> {
    application_state.panes.restore();
    Task::none()
}
pub fn pane_close(application_state: &mut ApplicationState, pane: Pane) -> Task<Message> {
    if let Some((_, sibling)) = application_state.panes.close(pane) {
        application_state.focus = Some(sibling);
    }
    Task::none()
}

pub fn pane_close_focussed(application_state: &mut ApplicationState) -> Task<Message> {
    if let Some(pane) = application_state.focus
        && let Some(PaneState { is_pinned, .. }) = application_state.panes.get(pane)
        && !is_pinned
        && let Some((_, sibling)) = application_state.panes.close(pane)
    {
        application_state.focus = Some(sibling);
    }
    Task::none()
}

pub fn pane_set_active_view(
    application_state: &mut ApplicationState,
    kind: InterpreterPaneViewKind,
    key: usize,
) -> Task<Message> {
    application_state.pane_purposes.insert(key, kind);
    Task::none()
}
