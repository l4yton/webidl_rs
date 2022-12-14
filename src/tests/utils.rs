use std::fs;

pub(super) fn load_test_file(name: &str) -> String {
    fs::read_to_string(&format!(
        "{}/resources/{}",
        env!("CARGO_MANIFEST_DIR"),
        name
    ))
    .expect("Failed to read asset file")
}
