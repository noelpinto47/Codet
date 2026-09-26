use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub struct OpenTab {
    pub path: PathBuf,
    pub label: String,   // filename only, for display
}

impl OpenTab {
    pub fn new(path: PathBuf) -> Self {
        let label = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| "untitled".to_string());
        Self { path, label }
    }
}