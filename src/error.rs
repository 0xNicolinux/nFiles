use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OrganizerError {
    #[error("Directory not found: {0}")]
    DirectoryNotFound(PathBuf),

    #[error("Path is not a directory: {0}")]
    NotADirectory(PathBuf),

    #[error("Permission denied: {0}")]
    PermissionDenied(PathBuf),

    #[error("Path traversal or target outside root detected for: {0}")]
    PathTraversal(PathBuf),

    #[error("I/O error at {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Invalid filename: {0}")]
    InvalidFilename(PathBuf),
}
