use std::path::PathBuf;

use crate::util::path_util::path_warden;

#[test]
fn test_path_warden() {
    let test_query_1 = PathBuf::from("../../../path");
    let test_query_2 = PathBuf::from("%2e%2e/%2e%2e/%2e%2e/etc/passwd");
    let test_query_3 = PathBuf::from("%2e%2e%2f%2e%2e%2f%2e%2e%2fetc/passwd");
    let test_query_4 = PathBuf::from("tech/icedlab/advagri");

    assert_eq!(path_warden("test".to_string(), test_query_1), None);
    assert_eq!(path_warden("test".to_string(), test_query_2), None);
    assert_eq!(path_warden("test".to_string(), test_query_3), None);
    assert_ne!(path_warden("test".to_string(), test_query_4), None);
}
