use crate::error::{Result, WMMSCoreError};

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Hash,Debug)]
pub struct CanonicalKey(String);

impl CanonicalKey {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn from_path(input: &str) -> Result<Self> {
        let mut s = input.trim().replace('\\', "/");

        // Collapse multiple slashes
        while s.contains("//") {
            s = s.replace("//", "/");
        }

        // Removes typical extensions
        if let Some(stripped) = s.strip_suffix(".alemb") {
            s = stripped.to_string();
        }

        // Convert file layout to dottet namespace
        s = s.trim_matches('/').to_string();

        if s.is_empty() {
            return Err(WMMSCoreError::InvalidPath(input.to_string()));
        }

        Ok(CanonicalKey(s))
    }

    pub fn from_dotted_ident(input: &str) -> Result<Self> {
        let s = input.trim();
        if s.is_empty() {
            return Err(WMMSCoreError::InvalidPath(input.to_string()));
        }

        let parts: Vec<&str> = s.split('.').collect();
        if parts.iter().any(|p| p.is_empty()) {
            return Err(WMMSCoreError::InvalidPath(input.to_string()));
        }

        Ok(CanonicalKey(parts.join(".")))
    }
}

// Canonical BTree Key Container
pub type CanonMap<K,V> = std::collections::BTreeMap<K, V>;
pub type CanonSet<K> = std::collections::BTreeSet<K>;

pub fn canon_sort<T: Ord>(v: &mut [T]) {
    v.sort();
}