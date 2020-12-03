use std::{
    fmt::Display,
    path::{Path, PathBuf},
    str::FromStr,
};

use once_cell::sync::Lazy;

pub use anyhow::{Context, Result};
pub use regex::Regex;

pub static CLI_PATH: Lazy<PathBuf> =
    Lazy::new(|| Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf());
pub static PROJECT_PATH: Lazy<PathBuf> = Lazy::new(|| CLI_PATH.join("..").canonicalize().unwrap());

#[derive(Debug, Clone, Copy, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct PocId(u16);

impl FromStr for PocId {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 4 && s.chars().all(char::is_numeric) {
            Ok(PocId(FromStr::from_str(s).unwrap()))
        } else {
            Err("Invalid PoC ID (4 digits required)")
        }
    }
}

impl PocId {
    pub fn iter_all() -> impl Iterator<Item = PocId> {
        (0..10000u16).map(|i| PocId(i))
    }
}

impl Display for PocId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04}", self.0)
    }
}
