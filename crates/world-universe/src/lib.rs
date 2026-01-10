use std::{error::Error, path::{Path, PathBuf}};
use walkdir::WalkDir;


pub mod universe;

/// Walks the directory tree starting from the given base path for a universe description file (universe.toml).
pub fn search_universe_manifest(base: &Path) -> Result<PathBuf, Box<dyn Error>> {
    let universe_desc_file = "universe.toml";
    for entry in WalkDir::new(base).into_iter().filter_map(Result::ok) {
        if entry.file_name() == universe_desc_file {
            return Ok(entry.path().to_path_buf());
        }
    }
    Err(format!("Universe description file '{}' not found in {:?}", universe_desc_file, base).into())
    
}

/// Check for universe manifest in current working directory
pub fn find_universe_manifest() -> Result<PathBuf, Box<dyn Error>> {
    let current_dir = std::env::current_dir()?;
    search_universe_manifest(&current_dir)
}