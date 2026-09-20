use crate::categories::Category;
use std::collections::HashMap;

pub fn default_extension_mappings() -> HashMap<String, Category> {
    let mut map = HashMap::new();

    // Images
    for ext in &["jpg", "jpeg", "png", "gif", "webp", "bmp", "svg", "tiff", "tif", "ico"] {
        map.insert((*ext).to_string(), Category::Images);
    }

    // Documents
    for ext in &["pdf", "doc", "docx", "odt", "txt", "rtf", "pages"] {
        map.insert((*ext).to_string(), Category::Documents);
    }

    // Spreadsheets
    for ext in &["xls", "xlsx", "ods", "csv"] {
        map.insert((*ext).to_string(), Category::Spreadsheets);
    }

    // Presentations
    for ext in &["ppt", "pptx", "odp"] {
        map.insert((*ext).to_string(), Category::Presentations);
    }

    // Audio
    for ext in &["mp3", "wav", "flac", "ogg", "m4a", "aac", "wma"] {
        map.insert((*ext).to_string(), Category::Audio);
    }

    // Video
    for ext in &["mp4", "mkv", "avi", "mov", "webm", "wmv", "m4v"] {
        map.insert((*ext).to_string(), Category::Videos);
    }

    // Archives
    for ext in &["zip", "rar", "7z", "tar", "gz", "bz2", "xz"] {
        map.insert((*ext).to_string(), Category::Archives);
    }

    // Executables / Installers (Windows, Linux, macOS)
    for ext in &[
        "exe", "msi", "bat", "cmd", // Windows
        "deb", "rpm", "appimage", "sh", // Linux
        "dmg", "pkg", "app", // macOS
    ] {
        map.insert((*ext).to_string(), Category::Applications);
    }

    // Source Code / Development Files
    for ext in &[
        "rs", "c", "cpp", "h", "hpp", "java", "py", "js", "ts", "jsx", "tsx",
        "go", "php", "rb", "swift", "kt", "kts", "json", "yaml", "yml", "toml",
        "xml", "html", "css", "scss", "sql",
    ] {
        map.insert((*ext).to_string(), Category::Code);
    }

    // Disk Images
    for ext in &["iso", "img"] {
        map.insert((*ext).to_string(), Category::DiskImages);
    }

    map
}

#[derive(Debug, Clone)]
pub struct Config {
    pub custom_mappings: HashMap<String, Category>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            custom_mappings: default_extension_mappings(),
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_category(&self, extension: &str) -> Option<&Category> {
        let ext_lower = extension.to_lowercase();
        self.custom_mappings.get(&ext_lower)
    }

    pub fn set_mapping(&mut self, extension: impl Into<String>, category: Category) {
        let ext = extension.into().to_lowercase();
        self.custom_mappings.insert(ext, category);
    }
}
