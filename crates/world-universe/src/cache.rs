use std::path::{ PathBuf};


pub struct CachePaths{
    pub compiled_assets: PathBuf,
    pub index_db: PathBuf,
    pub fingerprints: PathBuf,
    pub manifest: PathBuf,
}

pub struct CacheManager {
    pub cache_dir: PathBuf,
}

impl CacheManager {
    pub fn new(cache_dir: PathBuf) -> Self {
        CacheManager { cache_dir }
    }

    pub fn load_manifest(&self) -> Option<PathBuf> {
        // TODO: Implement loading of cache manifest
        None
    }

    pub fn save_manifest(&self, _manifest_path: &PathBuf) {
        // TODO: Implement saving of cache manifest
    }

    pub fn purge(&self) {
        // TODO: Implement purging of cache
    }
}