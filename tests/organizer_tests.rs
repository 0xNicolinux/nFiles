use file_organizer::categories::Category;
use file_organizer::organizer::{Organizer, OrganizerOptions};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_organizer_basic_flow() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    let img_file = root.join("photo.jpg");
    let doc_file = root.join("document.pdf");
    let unknown_file = root.join("unknown.xyz");

    fs::write(&img_file, "image data").unwrap();
    fs::write(&doc_file, "doc data").unwrap();
    fs::write(&unknown_file, "unknown data").unwrap();

    let options = OrganizerOptions {
        root_dir: root.to_path_buf(),
        recursive: false,
        dry_run: false,
        verbose: true,
    };

    let organizer = Organizer::default();
    let ops = organizer.scan(&options).unwrap();
    assert_eq!(ops.len(), 3);

    let summary = organizer.execute(&options, &ops);
    assert_eq!(summary.moved, 3);
    assert_eq!(summary.failed, 0);

    assert!(root.join("Images/photo.jpg").exists());
    assert!(root.join("Documents/document.pdf").exists());
    assert!(root.join("Others/unknown.xyz").exists());
    assert!(!img_file.exists());
}

#[test]
fn test_organizer_dry_run() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    let img_file = root.join("photo.jpg");
    fs::write(&img_file, "image data").unwrap();

    let options = OrganizerOptions {
        root_dir: root.to_path_buf(),
        recursive: false,
        dry_run: true,
        verbose: false,
    };

    let organizer = Organizer::default();
    let ops = organizer.scan(&options).unwrap();
    let summary = organizer.execute(&options, &ops);

    assert_eq!(summary.total_found, 1);
    assert!(img_file.exists());
    assert!(!root.join("Images/photo.jpg").exists());
}

#[test]
fn test_organizer_collision_handling() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    let images_dir = root.join("Images");
    fs::create_dir_all(&images_dir).unwrap();

    // Existing file in destination
    let existing = images_dir.join("photo.jpg");
    fs::write(&existing, "existing").unwrap();

    // Source file to move
    let src = root.join("photo.jpg");
    fs::write(&src, "new").unwrap();

    let options = OrganizerOptions {
        root_dir: root.to_path_buf(),
        recursive: false,
        dry_run: false,
        verbose: false,
    };

    let organizer = Organizer::default();
    let ops = organizer.scan(&options).unwrap();
    let summary = organizer.execute(&options, &ops);

    assert_eq!(summary.moved, 1);
    assert!(images_dir.join("photo.jpg").exists());
    assert!(images_dir.join("photo (1).jpg").exists());
}

#[test]
fn test_organizer_recursive_and_category_dir_exclusion() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    let sub_dir = root.join("subfolder");
    fs::create_dir_all(&sub_dir).unwrap();
    let sub_file = sub_dir.join("song.mp3");
    fs::write(&sub_file, "audio").unwrap();

    // Create a pre-existing Images category folder containing a file
    let images_dir = root.join("Images");
    fs::create_dir_all(&images_dir).unwrap();
    let existing_img = images_dir.join("already_organized.png");
    fs::write(&existing_img, "img").unwrap();

    let options = OrganizerOptions {
        root_dir: root.to_path_buf(),
        recursive: true,
        dry_run: false,
        verbose: true,
    };

    let organizer = Organizer::default();
    let ops = organizer.scan(&options).unwrap();

    // Should only pick up song.mp3 from subfolder, NOT files inside Images/
    assert_eq!(ops.len(), 1);
    assert_eq!(ops[0].category, Category::Audio);

    let summary = organizer.execute(&options, &ops);
    assert_eq!(summary.moved, 1);
    assert!(root.join("Audio/song.mp3").exists());
}
