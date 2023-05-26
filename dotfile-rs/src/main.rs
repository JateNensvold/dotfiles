use directories::BaseDirs;
use regex::Regex;
use std::env::{var, VarError};
use std::path::{Path, PathBuf};

fn main() {
    println!("Hello, world!");

    let dotfile = DotFile::default();
}

struct EnvironmentVariableNames {}

impl EnvironmentVariableNames {
    const SHELL: &'static str = "SHELL";
}

struct DotFile {
    ignore: Regex,
    shell: String,
    home: PathBuf,
    root: PathBuf,
}

impl Default for DotFile {
    fn default() -> Self {
        let base_dir = BaseDirs::new()
            .expect("Unable to determine location of common system directories such as $HOME");

        Self {
            ignore: Regex::new(r"").unwrap(),
            shell: var(EnvironmentVariableNames::SHELL).unwrap_or(String::from("Unknown")),
            home: base_dir.home_dir().to_path_buf(),
            root: Path::new("").to_path_buf(),
        }
    }
}
