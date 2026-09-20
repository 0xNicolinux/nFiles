use crate::categories::Category;
use std::path::{Path, PathBuf};

pub trait SystemDirectoryProvider {
    fn picture_dir(&self) -> Option<PathBuf>;
    fn document_dir(&self) -> Option<PathBuf>;
    fn audio_dir(&self) -> Option<PathBuf>;
    fn video_dir(&self) -> Option<PathBuf>;
}

#[derive(Default, Debug, Clone, Copy)]
pub struct DefaultSystemDirectoryProvider;

impl SystemDirectoryProvider for DefaultSystemDirectoryProvider {
    fn picture_dir(&self) -> Option<PathBuf> {
        dirs::picture_dir()
    }

    fn document_dir(&self) -> Option<PathBuf> {
        dirs::document_dir()
    }

    fn audio_dir(&self) -> Option<PathBuf> {
        dirs::audio_dir()
    }

    fn video_dir(&self) -> Option<PathBuf> {
        dirs::video_dir()
    }
}

pub struct DestinationResolver<P: SystemDirectoryProvider = DefaultSystemDirectoryProvider> {
    provider: P,
    use_system_dirs: bool,
}

impl DestinationResolver<DefaultSystemDirectoryProvider> {
    pub fn new(use_system_dirs: bool) -> Self {
        Self {
            provider: DefaultSystemDirectoryProvider,
            use_system_dirs,
        }
    }
}

impl<P: SystemDirectoryProvider> DestinationResolver<P> {
    pub fn with_provider(provider: P, use_system_dirs: bool) -> Self {
        Self {
            provider,
            use_system_dirs,
        }
    }

    pub fn resolve(&self, root_dir: &Path, category: &Category) -> PathBuf {
        if !self.use_system_dirs {
            return root_dir.join(category.directory_name());
        }

        let system_dir = match category {
            Category::Images => self.provider.picture_dir(),
            Category::Documents | Category::Spreadsheets | Category::Presentations => {
                self.provider.document_dir()
            }
            Category::Audio => self.provider.audio_dir(),
            Category::Videos => self.provider.video_dir(),
            Category::Archives
            | Category::Applications
            | Category::Code
            | Category::DiskImages
            | Category::Others => None,
        };

        system_dir.unwrap_or_else(|| root_dir.join(category.directory_name()))
    }
}
