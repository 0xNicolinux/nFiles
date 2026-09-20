use crate::categories::Category;
use crate::classifier::Classifier;
use crate::destination::{DefaultSystemDirectoryProvider, DestinationResolver, SystemDirectoryProvider};
use crate::error::OrganizerError;
use crate::filesystem::FilesystemHandler;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct MoveOperation {
    pub src_path: PathBuf,
    pub relative_src: PathBuf,
    pub category: Category,
    pub dest_dir: PathBuf,
    pub is_system_dest: bool,
}

#[derive(Debug, Default, Clone)]
pub struct Summary {
    pub total_found: usize,
    pub category_counts: HashMap<Category, usize>,
    pub moved: usize,
    pub skipped: usize,
    pub failed: usize,
    pub unrecognized: usize,
    pub errors: Vec<(PathBuf, String)>,
}

pub struct OrganizerOptions {
    pub root_dir: PathBuf,
    pub recursive: bool,
    pub dry_run: bool,
    pub verbose: bool,
    pub system_directories: bool,
}

pub struct Organizer<P: SystemDirectoryProvider = DefaultSystemDirectoryProvider> {
    classifier: Classifier,
    resolver: DestinationResolver<P>,
}

impl Default for Organizer<DefaultSystemDirectoryProvider> {
    fn default() -> Self {
        Self::new(Classifier::default(), DestinationResolver::new(false))
    }
}

impl Organizer<DefaultSystemDirectoryProvider> {
    pub fn with_system_directories(use_system_dirs: bool) -> Self {
        Self::new(
            Classifier::default(),
            DestinationResolver::new(use_system_dirs),
        )
    }
}

impl<P: SystemDirectoryProvider> Organizer<P> {
    pub fn new(classifier: Classifier, resolver: DestinationResolver<P>) -> Self {
        Self { classifier, resolver }
    }

    /// Scans the target directory and gathers all candidate files for organization.
    pub fn scan(&self, options: &OrganizerOptions) -> Result<Vec<MoveOperation>, OrganizerError> {
        let root = &options.root_dir;
        let category_dirs = FilesystemHandler::category_directory_names();

        let mut operations = Vec::new();
        let mut stack = vec![root.clone()];

        while let Some(current_dir) = stack.pop() {
            let entries = match fs::read_dir(&current_dir) {
                Ok(e) => e,
                Err(err) => {
                    eprintln!("Warning: Could not read directory {}: {}", current_dir.display(), err);
                    continue;
                }
            };

            for entry in entries {
                let entry = match entry {
                    Ok(e) => e,
                    Err(err) => {
                        eprintln!("Warning: Could not read entry in {}: {}", current_dir.display(), err);
                        continue;
                    }
                };

                let path = entry.path();
                let file_type = match entry.file_type() {
                    Ok(ft) => ft,
                    Err(err) => {
                        eprintln!("Warning: Could not get file type for {}: {}", path.display(), err);
                        continue;
                    }
                };

                // Do not follow symlinks by default
                if file_type.is_symlink() {
                    if options.verbose {
                        println!("Skipping symlink: {}", path.display());
                    }
                    continue;
                }

                if file_type.is_dir() {
                    let dir_name = match path.file_name().and_then(|n| n.to_str()) {
                        Some(name) => name,
                        None => continue,
                    };

                    // Prevent scanning category destination directories created by organizer
                    if category_dirs.contains(dir_name) {
                        if options.verbose {
                            println!("Skipping organizer category directory: {}", path.display());
                        }
                        continue;
                    }

                    if options.recursive {
                        stack.push(path);
                    }
                } else if file_type.is_file() {
                    let category = self.classifier.classify(&path);
                    let dest_dir = self.resolver.resolve(root, &category);
                    let is_system_dest = options.system_directories && dest_dir != root.join(category.directory_name());

                    let relative_src = path
                        .strip_prefix(root)
                        .map(|p| p.to_path_buf())
                        .unwrap_or_else(|_| path.clone());

                    operations.push(MoveOperation {
                        src_path: path,
                        relative_src,
                        category,
                        dest_dir,
                        is_system_dest,
                    });
                }
            }
        }

        // Sort for predictable execution order
        operations.sort_by(|a, b| a.src_path.cmp(&b.src_path));
        Ok(operations)
    }

    /// Generates a summary count of scanned operations.
    pub fn build_summary(&self, operations: &[MoveOperation]) -> Summary {
        let mut summary = Summary {
            total_found: operations.len(),
            ..Default::default()
        };

        for op in operations {
            *summary.category_counts.entry(op.category.clone()).or_insert(0) += 1;
            if op.category == Category::Others {
                summary.unrecognized += 1;
            }
        }

        summary
    }

    /// Executes the move operations according to the provided options.
    pub fn execute(&self, options: &OrganizerOptions, operations: &[MoveOperation]) -> Summary {
        let mut summary = self.build_summary(operations);

        if options.dry_run {
            println!("\n[DRY RUN]\n");
            for op in operations {
                let file_name = op
                    .src_path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("");
                let target_path = op.dest_dir.join(file_name);

                if op.is_system_dest {
                    println!(
                        "{}\n    -> {}",
                        op.relative_src.display(),
                        target_path.display()
                    );
                } else {
                    let rel_dest = Path::new(op.category.directory_name()).join(file_name);
                    println!(
                        "{:<25} -> {}",
                        op.relative_src.display(),
                        rel_dest.display()
                    );
                }
            }
            return summary;
        }

        println!("Organizing {}...", options.root_dir.display());
        println!();

        for op in operations {
            // Ensure destination directory exists
            if !op.dest_dir.exists() {
                if let Err(err) = fs::create_dir_all(&op.dest_dir) {
                    summary.failed += 1;
                    let msg = format!("Failed to create directory: {}", err);
                    eprintln!("Error: Permission denied / directory creation error while processing: {}\n  {}", op.src_path.display(), msg);
                    summary.errors.push((op.src_path.clone(), msg));
                    continue;
                }
            }

            // Verify safety check before moving
            if let Err(err) = FilesystemHandler::ensure_within_root(&options.root_dir, &op.src_path) {
                summary.failed += 1;
                let msg = format!("Path traversal safety check failed: {}", err);
                eprintln!("Error: Unsafe path detected: {}\n  The file was not modified.", op.src_path.display());
                summary.errors.push((op.src_path.clone(), msg));
                continue;
            }

            match FilesystemHandler::move_file(&op.src_path, &op.dest_dir) {
                Ok(final_dest) => {
                    summary.moved += 1;

                    if op.is_system_dest {
                        println!(
                            "✓ {:<25} → {}",
                            op.relative_src.display(),
                            final_dest.display()
                        );
                    } else if options.verbose {
                        let final_relative_dest = final_dest
                            .strip_prefix(&options.root_dir)
                            .map(|p| p.to_path_buf())
                            .unwrap_or(final_dest);
                        println!(
                            "✓ {:<25} → {} (Full path: {})",
                            op.relative_src.display(),
                            final_relative_dest.display(),
                            op.src_path.display()
                        );
                    } else {
                        println!(
                            "✓ {:<25} → {}/",
                            op.relative_src.display(),
                            op.category.directory_name()
                        );
                    }
                }
                Err(err) => {
                    summary.failed += 1;
                    let msg = match &err {
                        OrganizerError::PermissionDenied(_) => {
                            format!("Permission denied while moving: {}\nThe file was not modified.", op.src_path.display())
                        }
                        _ => format!("Error moving file: {}", err),
                    };
                    eprintln!("Error: {}\n  The file was not modified.", msg);
                    summary.errors.push((op.src_path.clone(), msg));
                }
            }
        }

        summary
    }
}
