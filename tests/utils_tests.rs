use diskdoc::utils::format_size;

#[test]
fn test_format_size() {
    assert_eq!(format_size(0), "0 B");
    assert_eq!(format_size(1024), "1 KiB");
    assert_eq!(format_size(1024 * 1024), "1 MiB");
    assert_eq!(format_size(1024 * 1024 * 1024), "1 GiB");
}
