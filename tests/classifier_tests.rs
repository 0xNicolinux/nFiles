use file_organizer::categories::Category;
use file_organizer::classifier::Classifier;
use std::path::Path;

#[test]
fn test_all_predefined_extensions() {
    let classifier = Classifier::default();

    // Images
    assert_eq!(classifier.classify(Path::new("file.jpg")), Category::Images);
    assert_eq!(classifier.classify(Path::new("file.jpeg")), Category::Images);
    assert_eq!(classifier.classify(Path::new("file.PNG")), Category::Images);
    assert_eq!(classifier.classify(Path::new("file.webp")), Category::Images);
    assert_eq!(classifier.classify(Path::new("file.svg")), Category::Images);

    // Documents
    assert_eq!(classifier.classify(Path::new("file.pdf")), Category::Documents);
    assert_eq!(classifier.classify(Path::new("file.docx")), Category::Documents);
    assert_eq!(classifier.classify(Path::new("file.txt")), Category::Documents);

    // Spreadsheets
    assert_eq!(classifier.classify(Path::new("file.xlsx")), Category::Spreadsheets);
    assert_eq!(classifier.classify(Path::new("file.csv")), Category::Spreadsheets);

    // Presentations
    assert_eq!(classifier.classify(Path::new("file.pptx")), Category::Presentations);

    // Audio
    assert_eq!(classifier.classify(Path::new("file.mp3")), Category::Audio);
    assert_eq!(classifier.classify(Path::new("file.flac")), Category::Audio);

    // Videos
    assert_eq!(classifier.classify(Path::new("file.mp4")), Category::Videos);
    assert_eq!(classifier.classify(Path::new("file.mkv")), Category::Videos);

    // Archives
    assert_eq!(classifier.classify(Path::new("file.zip")), Category::Archives);
    assert_eq!(classifier.classify(Path::new("file.tar")), Category::Archives);

    // Applications
    assert_eq!(classifier.classify(Path::new("file.exe")), Category::Applications);
    assert_eq!(classifier.classify(Path::new("file.AppImage")), Category::Applications);
    assert_eq!(classifier.classify(Path::new("file.dmg")), Category::Applications);

    // Code
    assert_eq!(classifier.classify(Path::new("file.rs")), Category::Code);
    assert_eq!(classifier.classify(Path::new("file.py")), Category::Code);
    assert_eq!(classifier.classify(Path::new("file.json")), Category::Code);

    // DiskImages
    assert_eq!(classifier.classify(Path::new("file.iso")), Category::DiskImages);
    assert_eq!(classifier.classify(Path::new("file.img")), Category::DiskImages);
}

#[test]
fn test_special_filenames() {
    let classifier = Classifier::default();

    // Spaces and Unicode
    assert_eq!(classifier.classify(Path::new("My Photo 2025 🔥.jpg")), Category::Images);
    assert_eq!(classifier.classify(Path::new("Relatório_Final.pdf")), Category::Documents);

    // Multiple dots
    assert_eq!(classifier.classify(Path::new("archive.tar.gz")), Category::Archives);
    assert_eq!(classifier.classify(Path::new("version.1.0.spec.rs")), Category::Code);

    // Unknown extensions and hidden files
    assert_eq!(classifier.classify(Path::new("file.unknown")), Category::Others);
    assert_eq!(classifier.classify(Path::new(".env")), Category::Others);
    assert_eq!(classifier.classify(Path::new("no_ext_file")), Category::Others);
}
