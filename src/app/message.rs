use crate::app::state::PanelTab;
use crate::widgets::sidebar::SidebarMessage;
use iced::widget::text_editor;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::fmt;
use portable_pty::MasterPty;

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
    ToggleFolder(Vec<usize>),
    OpenFile(PathBuf),
    SwitchTab(usize),
    CloseTab(usize),
    WindowDrag,
    WindowMinimize,
    WindowMaximize,
    WindowClose,
    TerminalOutput(String),
    TerminalInputChanged(String),
    TerminalInputSubmit,
    TerminalStarted(PtyHandle, WriterHandle),
}

#[derive(Clone)]
pub struct PtyHandle(pub Arc<Mutex<Box<dyn MasterPty + Send>>>);

impl fmt::Debug for PtyHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PtyHandle(<MasterPty>)")
    }
}

#[derive(Clone)]
pub struct WriterHandle(pub Arc<Mutex<Box<dyn std::io::Write + Send>>>);

impl fmt::Debug for WriterHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "WriterHandle(<Write>)")
    }
}