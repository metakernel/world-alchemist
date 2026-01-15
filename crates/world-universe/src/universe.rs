

use serde::{Deserialize, Serialize};

use crate::world_root::WorldRoot;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniverseManifest {
    pub world_name: String,
    pub version: Option<String>,
    pub authors: Option<Vec<String>>,
    pub default_scope: Option<String>,

}

impl UniverseManifest {
    pub fn new(name: &str, authors: Option<Vec<String>>) -> Self {
        Self {
            world_name: name.to_string(),
            version: None,
            authors,
            default_scope: None,
        }
    }
    pub fn load(_path: &WorldRoot) -> Result<Self, Box<dyn std::error::Error>> {
        // Placeholder for loading logic
        Ok(Self::new("DefaultUniverse", None))
    }
    pub fn save(&self, _root: WorldRoot) -> Result<(), Box<dyn std::error::Error>> {
        // Placeholder for saving logic
        Ok(())
    }
}