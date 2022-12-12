use std::{fmt, fs, path::Path};

pub(super) fn load_test_file<T: AsRef<Path> + fmt::Display>(name: T) -> String {
    fs::read_to_string(&format!(
        "{}/resources/{}",
        env!("CARGO_MANIFEST_DIR"),
        name
    ))
    .expect("Failed to read asset file")
}
