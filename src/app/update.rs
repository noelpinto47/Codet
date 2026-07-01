use iced::Task;

use crate::widgets::sidebar::SidebarMessage;

use super::{message::Message, state::App};

pub fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Sidebar(SidebarMessage::Pressed(item)) => {
            app.selected_sidebar = item;
        }
    }

    Task::none()
}