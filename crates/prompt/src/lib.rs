use std::fs;
use std::path::{Path, PathBuf};

pub fn get_prompt(type_arg: &str) -> String {
  let mut file_path = PathBuf::new();
  file_path.push(Path::new(env!("CARGO_MANIFEST_DIR")));
  file_path.push("template");
  file_path.push(format!("{}.txt", type_arg));
  match fs::read_to_string(file_path) {
    Ok(contents) => contents.replace("\r\n", "\\n"),
    Err(_) => "".to_string(),
  }
}
