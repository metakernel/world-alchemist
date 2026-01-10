use std::{error::Error, path::{Path, PathBuf}};

use walkdir::WalkDir;

use crate::error::{UniverseManifestNotFoundError, WorldRootNotFoundError};

pub struct WorldRoot {
    pub path: PathBuf,
}

impl WorldRoot {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn find_from_path(base: &Path) -> Result<Self, Box<dyn Error>> {
        for entry in WalkDir::new(base).into_iter().filter_map(Result::ok) {
        if entry.file_type().is_dir() && entry.file_name() == ".world" {
            let root = WorldRoot::new(entry.path().parent().unwrap_or(entry.path()).to_path_buf());
            // Check if universe.toml exists in this .world directory
            let manifest_path = root.path.join(".world").join("universe.toml");
            if manifest_path.exists() {
                return Ok(root);
            } else {
                return Err(UniverseManifestNotFoundError.into());
            }
        } 
    }
        Err(WorldRootNotFoundError.into())
    }

    pub fn discover() -> Result<Self, Box<dyn Error>> {
        let current_dir = std::env::current_dir()?;
        Self::find_from_path(&current_dir)
    }

    pub fn get_universe_manifest_path(&self) -> PathBuf {
        self.path.join(".world").join("universe.toml")
    }
}