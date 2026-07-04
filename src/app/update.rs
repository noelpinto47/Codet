use crate::{
    app::{message::Message, state::AppState},
    widgets::sidebar::{SidebarItem, SidebarMessage},
};

pub fn update(app: &mut AppState, message: Message) {
    match message {
        Message::EditorEdit(action) => {
            app.editor.perform(action);
        }

        Message::Sidebar(sidebar_message) => match sidebar_message {
            SidebarMessage::Pressed(item) => {
                if app.selected_sidebar == item {
                    app.sidebar_open = !app.sidebar_open;
                } else {
                    app.selected_sidebar = item;
                    app.sidebar_open = true;
                }

                if item == SidebarItem::Settings {
                    app.show_settings = true;
                } else {
                    app.selected_sidebar = item;
                    app.show_settings = false;
                }
            }
        },

        Message::CloseSettings => {
            app.show_settings = false;
        }
    }
}