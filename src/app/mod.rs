pub mod message;
pub mod state;
pub mod update;
pub mod view;

use iced::{window, Theme};

pub fn run() -> iced::Result {
    let icon = window::icon::from_file_data(include_bytes!("../../assets/icon.png"), None)
        .expect("failed to load application icon");

    iced::application(state::AppState::default, update::update, view::view)
        .title("Codet")
        .window(window::Settings {
            icon: Some(icon),
            ..Default::default()
        })
        .theme(Theme::Dark)
        .run()
}