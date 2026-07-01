use iced::Task;

use super::{message::Message, state::App};

pub fn update(_app: &mut App, _message: Message) -> Task<Message> {
    Task::none()
}