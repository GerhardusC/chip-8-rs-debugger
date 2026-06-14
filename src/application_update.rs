use iced::widget::pane_grid;

use crate::{ApplicationState, InterpreterPaneViewKind};

#[derive(Debug, Clone)]
pub enum Message {
    PaneSetActiveView(InterpreterPaneViewKind, usize),
    NextInstruction,
    KeyToggled(u8),
    TempLoadProgram,
    ToggleRunning,
    Tick,
    PaneSplit(pane_grid::Axis, pane_grid::Pane),
    PaneSplitFocused(pane_grid::Axis),
    PaneFocusAdjacent(pane_grid::Direction),
    PaneClicked(pane_grid::Pane),
    PaneDragged(pane_grid::DragEvent),
    PaneResized(pane_grid::ResizeEvent),
    PaneTogglePin(pane_grid::Pane),
    PaneMaximize(pane_grid::Pane),
    PaneRestore,
    PaneClose(pane_grid::Pane),
    PaneCloseFocused,
}

#[derive(Clone, Copy, Default)]
pub struct Pane {
    pub id: usize,
    pub is_pinned: bool,
}

impl Pane {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            is_pinned: false,
        }
    }
}

pub fn application_update(application_state: &mut ApplicationState, message: Message) {
    match message {
        Message::NextInstruction => {
            let emulator_state = application_state.emulator.0.borrow_mut().next();
            if let Some(emulator_state) = emulator_state {
                application_state.emulator_state = emulator_state;
            }
        }
        Message::TempLoadProgram => {
            let program = std::fs::read("/home/gerhardus/Downloads/Pong (1 player).ch8");
            if let Ok(program) = program {
                let _ = application_state.emulator.0.borrow_mut().reset(program);
            }
        }
        Message::Tick => {
            let emulator_state = application_state.emulator.0.borrow_mut().next();
            if let Some(emulator_state) = emulator_state {
                application_state.emulator_state = emulator_state;
            }
        }
        Message::ToggleRunning => {
            application_state.is_running = !application_state.is_running;
        }
        Message::KeyToggled(key) => {
            if let Some(key) = application_state
                .emulator
                .0
                .borrow_mut()
                .input_provider
                .keys_state
                .get_mut(key as usize & 0xF)
            {
                *key = if *key > 0 { 0 } else { 1 };
            };
        }
        Message::PaneSplit(axis, pane) => {
            let result = application_state.panes.split(
                axis,
                pane,
                Pane::new(application_state.panes_created),
            );

            if let Some((pane, _)) = result {
                application_state.focus = Some(pane);
            }

            application_state.panes_created += 1;
        }
        Message::PaneSplitFocused(axis) => {
            if let Some(pane) = application_state.focus {
                let result = application_state.panes.split(
                    axis,
                    pane,
                    Pane::new(application_state.panes_created),
                );

                if let Some((pane, _)) = result {
                    application_state.focus = Some(pane);
                }

                application_state.panes_created += 1;
            }
        }
        Message::PaneFocusAdjacent(direction) => {
            if let Some(pane) = application_state.focus
                && let Some(adjacent) = application_state.panes.adjacent(pane, direction)
            {
                application_state.focus = Some(adjacent);
            }
        }
        Message::PaneClicked(pane) => {
            application_state.focus = Some(pane);
        }
        Message::PaneResized(pane_grid::ResizeEvent { split, ratio }) => {
            application_state.panes.resize(split, ratio);
        }
        Message::PaneDragged(pane_grid::DragEvent::Dropped { pane, target }) => {
            application_state.panes.drop(pane, target);
        }
        Message::PaneDragged(_) => {}
        Message::PaneTogglePin(pane) => {
            if let Some(Pane { is_pinned, .. }) = application_state.panes.get_mut(pane) {
                *is_pinned = !*is_pinned;
            }
        }
        Message::PaneMaximize(pane) => application_state.panes.maximize(pane),
        Message::PaneRestore => {
            application_state.panes.restore();
        }
        Message::PaneClose(pane) => {
            if let Some((_, sibling)) = application_state.panes.close(pane) {
                application_state.focus = Some(sibling);
            }
        }
        Message::PaneCloseFocused => {
            if let Some(pane) = application_state.focus
                && let Some(Pane { is_pinned, .. }) = application_state.panes.get(pane)
                && !is_pinned
                && let Some((_, sibling)) = application_state.panes.close(pane)
            {
                application_state.focus = Some(sibling);
            }
        }
        Message::PaneSetActiveView(interpreter_pane_view_kind, k) => {
            application_state
                .pane_purposes
                .insert(k, interpreter_pane_view_kind);
        }
    }
}
