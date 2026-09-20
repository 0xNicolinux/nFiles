use file_organizer::categories::Category;
use file_organizer::classifier::Classifier;
use file_organizer::destination::{DestinationResolver, SystemDirectoryProvider};
use file_organizer::organizer::{Organizer, OrganizerOptions};
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::tempdir;

struct MockSystemDirProvider {
    picture: Option<PathBuf>,
    document: Option<PathBuf>,
    audio: Option<PathBuf>,
    video: Option<PathBuf>,
}

impl SystemDirectoryProvider for MockSystemDirProvider {
    fn picture_dir(&self) -> Option<PathBuf> {
        self.picture.clone()
    }

    fn document_dir(&self) -> Option<PathBuf> {
        self.document.clone()
    }

    fn audio_dir(&self) -> Option<PathBuf> {
        self.audio.clone()
    }

    fn video_dir(&self) -> Option<PathBuf> {
        self.video.clone()
    }
}

#[test]
fn test_default_mode_uses_category_directories() {
    let mock = MockSystemDirProvider {
        picture: Some(PathBuf::from("/sys/Pictures")),
        document: Some(PathBuf::from("/sys/Documents")),
        audio: Some(PathBuf::from("/sys/Music")),
        video: Some(PathBuf::from("/sys/Videos")),
    };

    let root = Path::new("/user/Downloads");
    let resolver = DestinationResolver::with_provider(mock, false);

    assert_eq!(resolver.resolve(root, &Category::Images), root.join("Images"));
    assert_eq!(resolver.resolve(root, &Category::Documents), root.join("Documents"));
    assert_eq!(resolver.resolve(root, &Category::Spreadsheets), root.join("Spreadsheets"));
    assert_eq!(resolver.resolve(root, &Category::Presentations), root.join("Presentations"));
    assert_eq!(resolver.resolve(root, &Category::Audio), root.join("Audio"));
    assert_eq!(resolver.resolve(root, &Category::Videos), root.join("Videos"));
    assert_eq!(resolver.resolve(root, &Category::Archives), root.join("Archives"));
    assert_eq!(resolver.resolve(root, &Category::Others), root.join("Others"));
}

#[test]
fn test_system_directories_category_mappings() {
    let mock = MockSystemDirProvider {
        picture: Some(PathBuf::from("/sys/Pictures")),
        document: Some(PathBuf::from("/sys/Documents")),
        audio: Some(PathBuf::from("/sys/Music")),
        video: Some(PathBuf::from("/sys/Videos")),
    };

    let root = Path::new("/user/Downloads");
    let resolver = DestinationResolver::with_provider(mock, true);

    // Images -> Pictures
    assert_eq!(resolver.resolve(root, &Category::Images), PathBuf::from("/sys/Pictures"));

    // Documents, Spreadsheets, Presentations -> Documents
    assert_eq!(resolver.resolve(root, &Category::Documents), PathBuf::from("/sys/Documents"));
    assert_eq!(resolver.resolve(root, &Category::Spreadsheets), PathBuf::from("/sys/Documents"));
    assert_eq!(resolver.resolve(root, &Category::Presentations), PathBuf::from("/sys/Documents"));

    // Audio -> Music/Audio
    assert_eq!(resolver.resolve(root, &Category::Audio), PathBuf::from("/sys/Music"));

    // Videos -> Video
    assert_eq!(resolver.resolve(root, &Category::Videos), PathBuf::from("/sys/Videos"));

    // Without system equivalents -> Local category directory
    assert_eq!(resolver.resolve(root, &Category::Archives), root.join("Archives"));
    assert_eq!(resolver.resolve(root, &Category::Applications), root.join("Applications"));
    assert_eq!(resolver.resolve(root, &Category::Code), root.join("Code"));
    assert_eq!(resolver.resolve(root, &Category::DiskImages), root.join("DiskImages"));
    assert_eq!(resolver.resolve(root, &Category::Others), root.join("Others"));
}

#[test]
fn test_system_directories_fallback_when_none() {
    let mock = MockSystemDirProvider {
        picture: None,
        document: None,
        audio: None,
        video: None,
    };

    let root = Path::new("/user/Downloads");
    let resolver = DestinationResolver::with_provider(mock, true);

    assert_eq!(resolver.resolve(root, &Category::Images), root.join("Images"));
    assert_eq!(resolver.resolve(root, &Category::Documents), root.join("Documents"));
    assert_eq!(resolver.resolve(root, &Category::Audio), root.join("Audio"));
    assert_eq!(resolver.resolve(root, &Category::Videos), root.join("Videos"));
}

#[test]
fn test_dry_run_reports_system_destinations_without_moving() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    let sys_pics = dir.path().join("SystemPictures");
    fs::create_dir_all(&sys_pics).unwrap();

    let mock = MockSystemDirProvider {
        picture: Some(sys_pics.clone()),
        document: None,
        audio: None,
        video: None,
    };

    let img_file = root.join("photo.jpg");
    fs::write(&img_file, "data").unwrap();

    let options = OrganizerOptions {
        root_dir: root.to_path_buf(),
        recursive: false,
        dry_run: true,
        verbose: false,
        system_directories: true,
    };

    let resolver = DestinationResolver::with_provider(mock, true);
    let organizer = Organizer::new(Classifier::default(), resolver);

    let ops = organizer.scan(&options).unwrap();
    assert_eq!(ops.len(), 1);
    assert_eq!(ops[0].dest_dir, sys_pics);
    assert!(ops[0].is_system_dest);

    let summary = organizer.execute(&options, &ops);

    assert_eq!(summary.total_found, 1);
    assert!(img_file.exists());
    assert!(!sys_pics.join("photo.jpg").exists());
}
