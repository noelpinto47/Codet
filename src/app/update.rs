use iced::Task;

use crate::widgets::sidebar::SidebarMessage;

use super::{message::Message, state::App};

pub fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Sidebar(SidebarMessage::Pressed(item)) => {
            if app.selected_sidebar == item {
                app.sidebar_open = !app.sidebar_open;
            } else {
                app.selected_sidebar = item;
                app.sidebar_open = true;
            }
        }
    }

    Task::none()
}