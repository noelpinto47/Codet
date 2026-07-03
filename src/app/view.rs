use iced::Element;

use super::{message::Message, state::AppState};

pub fn view(app: &AppState) -> Element<'_, Message> {
    crate::views::editor::view(app)
}