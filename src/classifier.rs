use crate::categories::Category;
use crate::config::Config;
use std::path::Path;

pub struct Classifier {
    config: Config,
}

impl Default for Classifier {
    fn default() -> Self {
        Self::new(Config::default())
    }
}

impl Classifier {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Classifies a path into a Category based on its file extension.
    ///
    /// If the path has no extension or its extension is not recognized,
    /// it returns `Category::Others`.
    pub fn classify(&self, path: &Path) -> Category {
        let extension = path.extension().and_then(|e| e.to_str());

        if let Some(ext) = extension {
            if let Some(category) = self.config.get_category(ext) {
                return category.clone();
            }
        }

        Category::Others
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_classify_standard_extensions() {
        let classifier = Classifier::default();

        assert_eq!(classifier.classify(Path::new("photo.jpg")), Category::Images);
        assert_eq!(classifier.classify(Path::new("photo.JPEG")), Category::Images);
        assert_eq!(classifier.classify(Path::new("doc.pdf")), Category::Documents);
        assert_eq!(classifier.classify(Path::new("song.mp3")), Category::Audio);
        assert_eq!(classifier.classify(Path::new("movie.mp4")), Category::Videos);
        assert_eq!(classifier.classify(Path::new("archive.zip")), Category::Archives);
        assert_eq!(classifier.classify(Path::new("main.rs")), Category::Code);
        assert_eq!(classifier.classify(Path::new("setup.exe")), Category::Applications);
        assert_eq!(classifier.classify(Path::new("app.AppImage")), Category::Applications);
        assert_eq!(classifier.classify(Path::new("disk.iso")), Category::DiskImages);
    }

    #[test]
    fn test_classify_unknown_and_no_extension() {
        let classifier = Classifier::default();

        assert_eq!(classifier.classify(Path::new("file.unknown_ext")), Category::Others);
        assert_eq!(classifier.classify(Path::new("README")), Category::Others);
        assert_eq!(classifier.classify(Path::new(".gitignore")), Category::Others);
    }
}
