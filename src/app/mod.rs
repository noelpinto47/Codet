pub mod message;
pub mod state;
pub mod update;
pub mod view;

use iced::Theme;

pub fn run() -> iced::Result {
    iced::application(state::AppState::default, update::update, view::view)
        .title("Codet")
        .theme(Theme::Dark)
        .run()
}