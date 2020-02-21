use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{ffi::OsStr, fs};
use toml;

#[derive(Debug, Deserialize, Serialize)]
pub struct LangConfig {
    #[serde(default = "use_spaces")]
    pub use_spaces: bool,
    #[serde(default = "space_size")]
    pub space_size: usize,
    #[serde(default = "newlines_at_end")]
    pub newlines_at_end: usize,
}

fn use_spaces() -> bool {
    true
}

fn space_size() -> usize {
    4
}

fn newlines_at_end() -> usize {
    1
}

impl Default for LangConfig {
    fn default() -> Self {
        LangConfig {
            use_spaces: true,
            space_size: 4,
            newlines_at_end: 1,
        }
    }
}

impl LangConfig {
    pub fn new(input_path: &PathBuf) -> LangConfig {
        let names = vec![
            OsStr::new("gml_fmt.toml"),
            OsStr::new(".gml_fmt.toml"),
            OsStr::new(".gml_fmt"),
        ];

        for entry in fs::read_dir(input_path).expect(&format!("Error reading directory {:?}", input_path)) {
            let entry = entry.expect(&format!("Error reading file"));
            let path = entry.path();

            if path.is_file() {
                let fname = path.file_name().expect("Error reading filename.");

                if names.contains(&fname) {
                    let lang_config: LangConfig = toml::from_str(&fs::read_to_string(path).unwrap()).unwrap();
                    return lang_config;
                }
            }
        }

        LangConfig {
            newlines_at_end: 1,
            use_spaces: true,
            space_size: 4,
        }
    }
}
