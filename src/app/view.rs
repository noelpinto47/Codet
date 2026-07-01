use iced::Element;

use super::{message::Message, state::App};

pub fn view(_app: &App) -> Element<'_, Message> {
    crate::views::home::view()
}