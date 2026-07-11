use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum FileNode {
    File {
        name: String,
        path: PathBuf,
    },
    Folder {
        name: String,
        expanded: bool,
        children: Vec<FileNode>,
    },
}
