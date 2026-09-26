pub mod message;
pub mod state;
pub mod update;
pub mod view;
pub mod subscription;

use iced::{window, Size, Theme};

pub fn run() -> iced::Result {
    let icon = window::icon::from_file_data(include_bytes!("../../assets/icon.png"), None)
        .expect("failed to load application icon");

    iced::application(state::AppState::default, update::update, view::view)
        .title("Codet")
        .subscription(|_state| subscription::terminal_subscription())
        .window(window::Settings {
            icon: Some(icon),
            decorations: false,
            transparent: true,
            min_size: Some(Size::new(800.0, 500.0)),
            ..Default::default()
        })
        .theme(app_theme)
        .run()
}

fn app_theme(_state: &state::AppState) -> Theme {
    Theme::Dark
}
