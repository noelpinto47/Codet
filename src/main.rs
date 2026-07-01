use iced::{
    widget::{column, text, text_input},
    Element, Task,
};

pub fn main() -> iced::Result {
    iced::application(App::default, update, view)
        .title("My Iced App")
        .run()
}

#[derive(Default)]
struct App {
    name: String
}

#[derive(Debug, Clone)]
enum Message {
    NameChanged(String)
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::NameChanged(value) => {
            app.name = value;
        }
    }

    Task::none()
}

fn view(app: &App) -> Element<'_, Message> {
    let greeting = if app.name.is_empty() {
        text("Hello!")
    } else {
        text(format!("Hello, {}!", app.name))
    };

    column![
        text("Rust + Iced starter").size(32),
        greeting,
        text_input("Enter your name", &app.name)
            .on_input(Message::NameChanged)
            .padding(10),
    ]
    .spacing(16)
    .padding(24)
    .into()
}
