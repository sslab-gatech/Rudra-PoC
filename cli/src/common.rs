use std::collections::HashSet;
use std::fs;

use crate::prelude::*;

use once_cell::sync::Lazy;

pub fn get_poc_id_set() -> Result<HashSet<PocId>> {
    let mut id_set = HashSet::new();

    let poc_dir = PROJECT_PATH.join("poc");
    for entry in
        fs::read_dir(&poc_dir).with_context(|| format!("Failed to access {}", poc_dir.display()))?
    {
        static PATTERN: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"^(\d{4})-[A-Za-z0-9\-_]+\.rs$").unwrap());

        if let Ok(entry) = entry {
            if let Some(file_name) = entry.file_name().to_str() {
                if let Some(captures) = PATTERN.captures(file_name) {
                    let poc_id: PocId = captures.get(1).unwrap().as_str().parse().unwrap();

                    // Check no duplication
                    assert!(id_set.insert(poc_id));
                }
            }
        }
    }

    Ok(id_set)
}
