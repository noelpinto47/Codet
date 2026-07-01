use iced::Element;

use super::{message::Message, state::App};

pub fn view(app: &App) -> Element<'_, Message> {
    crate::views::home::view(app.selected_sidebar)
}