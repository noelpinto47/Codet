use iced::{
    Element,
    widget::{center, column, text},
};

use crate::app::message::Message;

pub fn view() -> Element<'static, Message> {
    center(
        column![
            text("Codet").size(40),
            text("Lightweight IDE, built with Rust and Iced.")
        ]
        .spacing(12)
    )
    .into()
}