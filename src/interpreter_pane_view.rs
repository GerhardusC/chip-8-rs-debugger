use iced::Element;
use iced::widget::{column, container, pick_list};

use crate::file_picker_view::file_picker;
use crate::{AVAILABLE_VIEWS, InterpreterPaneViewKind};
use crate::{
    ApplicationState, Message, controls_view::controls,
    interpreter_screen_view::interpreter_screen, keypad_view::keypad, metadata_view::metadata,
};

pub fn interpreter_pane(app_state: &'_ ApplicationState, id: usize) -> Element<'_, Message> {
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
