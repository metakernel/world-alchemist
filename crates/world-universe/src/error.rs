use std::fmt::{Display, Formatter};


#[derive(Debug)]
pub struct WorldRootNotFoundError;

impl Display for WorldRootNotFoundError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "World root directory not found")
    }
}

impl std::error::Error for WorldRootNotFoundError {}

#[derive(Debug)]
pub struct UniverseManifestNotFoundError;

impl Display for UniverseManifestNotFoundError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Universe manifest file not found")
    }
}

impl std::error::Error for UniverseManifestNotFoundError {}