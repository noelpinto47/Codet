use iced::Task;

use crate::widgets::sidebar::SidebarMessage;

use super::{message::Message, state::AppState};

pub fn update(app: &mut AppState, message: Message) -> Task<Message> {
    match message {
        Message::Sidebar(SidebarMessage::Pressed(item)) => {
            if app.selected_sidebar == item {
                app.sidebar_open = !app.sidebar_open;
            } else {
                app.selected_sidebar = item;
                app.sidebar_open = true;
            }
        }

        Message::EditorEdit(action) => {
                app.editor.perform(action);
        }
    }
    Task::none()
}