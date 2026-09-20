use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Category {
    Images,
    Documents,
    Spreadsheets,
    Presentations,
    Audio,
    Videos,
    Archives,
    Applications,
    Code,
    DiskImages,
    Others,
}

impl Category {
    pub fn directory_name(&self) -> &'static str {
        match self {
            Category::Images => "Images",
            Category::Documents => "Documents",
            Category::Spreadsheets => "Spreadsheets",
            Category::Presentations => "Presentations",
            Category::Audio => "Audio",
            Category::Videos => "Videos",
            Category::Archives => "Archives",
            Category::Applications => "Applications",
            Category::Code => "Code",
            Category::DiskImages => "DiskImages",
            Category::Others => "Others",
        }
    }

    pub fn all_categories() -> Vec<Category> {
        vec![
            Category::Images,
            Category::Documents,
            Category::Spreadsheets,
            Category::Presentations,
            Category::Audio,
            Category::Videos,
            Category::Archives,
            Category::Applications,
            Category::Code,
            Category::DiskImages,
            Category::Others,
        ]
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.directory_name())
    }
}
