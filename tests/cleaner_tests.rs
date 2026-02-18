use diskdoc::domain::ports::Cleaner;
use diskdoc::infrastructure::cleaner::FsCleaner;
use std::fs;
use std::path::PathBuf;

#[test]
fn test_delete_file() {
    let cleaner = FsCleaner::new();
    let path = PathBuf::from("test_delete_file.txt");
    fs::write(&path, "content").unwrap();
    assert!(path.exists());

    let size = cleaner.delete_item(&path).expect("Failed to delete file");
    assert_eq!(size, 7); // "content" is 7 bytes
    assert!(!path.exists());
}

#[test]
fn test_delete_directory() {
    let cleaner = FsCleaner::new();
    let dir = PathBuf::from("test_delete_dir");
    fs::create_dir(&dir).unwrap();
    let file = dir.join("file.txt");
    fs::write(&file, "content").unwrap();

    assert!(dir.exists());
    assert!(file.exists());

    let _ = cleaner
        .delete_item(&dir)
        .expect("Failed to delete directory");

    assert!(!file.exists());
    assert!(!dir.exists());
}
