use crate::categories::Category;
use crate::error::OrganizerError;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

pub struct FilesystemHandler;

impl FilesystemHandler {
    /// Normalizes and resolves a target path, expanding `~` if needed.
    pub fn resolve_path(path_str: &str) -> Result<PathBuf, OrganizerError> {
        let expanded = if path_str == "~" || path_str.starts_with("~/") || path_str.starts_with("~\\") {
            if let Some(home) = dirs::home_dir() {
                if path_str == "~" {
                    home
                } else {
                    home.join(&path_str[2..])
                }
            } else {
                PathBuf::from(path_str)
            }
        } else {
            PathBuf::from(path_str)
        };

        let canonical = expanded.canonicalize().map_err(|e| OrganizerError::Io {
            path: expanded.clone(),
            source: e,
        })?;

        if !canonical.exists() {
            return Err(OrganizerError::DirectoryNotFound(canonical));
        }

        if !canonical.is_dir() {
            return Err(OrganizerError::NotADirectory(canonical));
        }

        Ok(canonical)
    }

    /// Verifies that `target` is within or equal to `root_dir` to prevent path traversal.
    pub fn ensure_within_root(root_dir: &Path, target: &Path) -> Result<(), OrganizerError> {
        let root_canonical = root_dir.canonicalize().map_err(|e| OrganizerError::Io {
            path: root_dir.to_path_buf(),
            source: e,
        })?;

        let target_canonical = target.canonicalize().map_err(|e| OrganizerError::Io {
            path: target.to_path_buf(),
            source: e,
        })?;

        if !target_canonical.starts_with(&root_canonical) {
            return Err(OrganizerError::PathTraversal(target.to_path_buf()));
        }

        Ok(())
    }

    /// Generates a unique filename in `target_dir` if `file_name` already exists.
    /// Example: `photo.jpg` -> `photo (1).jpg` -> `photo (2).jpg`
    pub fn generate_unique_path(target_dir: &Path, file_name: &str) -> PathBuf {
        let mut dest = target_dir.join(file_name);
        if !dest.exists() {
            return dest;
        }

        let path = Path::new(file_name);
        let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or(file_name);
        let extension = path.extension().and_then(|e| e.to_str());

        let mut counter = 1;
        loop {
            let new_name = match extension {
                Some(ext) => format!("{} ({}).{}", stem, counter, ext),
                None => format!("{} ({})", stem, counter),
            };

            dest = target_dir.join(&new_name);
            if !dest.exists() {
                return dest;
            }
            counter += 1;
        }
    }

    /// Returns the set of directory names created for categories (e.g., "Images", "Documents", "Others").
    pub fn category_directory_names() -> HashSet<String> {
        Category::all_categories()
            .into_iter()
            .map(|cat| cat.directory_name().to_string())
            .collect()
    }

    /// Creates category directory if it doesn't exist yet.
    pub fn ensure_category_dir(root_dir: &Path, category: &Category) -> Result<PathBuf, OrganizerError> {
        let dir_path = root_dir.join(category.directory_name());
        if !dir_path.exists() {
            fs::create_dir_all(&dir_path).map_err(|e| OrganizerError::Io {
                path: dir_path.clone(),
                source: e,
            })?;
        }
        Ok(dir_path)
    }

    /// Safely moves a file from `src` to `dest_dir`, using collision resolution.
    ///
    /// Returns the final destination path where the file was moved.
    pub fn move_file(src: &Path, dest_dir: &Path) -> Result<PathBuf, OrganizerError> {
        let file_name = src
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| OrganizerError::InvalidFilename(src.to_path_buf()))?;

        let dest_path = Self::generate_unique_path(dest_dir, file_name);

        // Attempt rename first (atomic on same filesystem)
        if fs::rename(src, &dest_path).is_err() {
            // Fallback to copy + remove for cross-filesystem moves
            fs::copy(src, &dest_path).map_err(|e| OrganizerError::Io {
                path: src.to_path_buf(),
                source: e,
            })?;
            fs::remove_file(src).map_err(|e| OrganizerError::Io {
                path: src.to_path_buf(),
                source: e,
            })?;
        }

        Ok(dest_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_collision_handling() {
        let dir = tempdir().unwrap();
        let target_dir = dir.path();

        let file_name = "photo.jpg";
        let path1 = target_dir.join(file_name);
        fs::write(&path1, "data1").unwrap();

        let unique2 = FilesystemHandler::generate_unique_path(target_dir, file_name);
        assert_eq!(unique2, target_dir.join("photo (1).jpg"));
        fs::write(&unique2, "data2").unwrap();

        let unique3 = FilesystemHandler::generate_unique_path(target_dir, file_name);
        assert_eq!(unique3, target_dir.join("photo (2).jpg"));
    }

    #[test]
    fn test_collision_handling_no_extension() {
        let dir = tempdir().unwrap();
        let target_dir = dir.path();

        let file_name = "LICENSE";
        let path1 = target_dir.join(file_name);
        fs::write(&path1, "data1").unwrap();

        let unique2 = FilesystemHandler::generate_unique_path(target_dir, file_name);
        assert_eq!(unique2, target_dir.join("LICENSE (1)"));
    }
}
