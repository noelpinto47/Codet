use crate::app::state::PanelTab;
use crate::widgets::sidebar::SidebarMessage;
use iced::widget::text_editor;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Message {
    Sidebar(SidebarMessage),
    EditorEdit(text_editor::Action),
    CloseSettings,
    ToggleTerminal,
    OpenPanel(PanelTab),
    ClosePanel,
    ToggleFileMenu,
    OpenFolderClicked,
    FolderOpened(Option<PathBuf>),
}
