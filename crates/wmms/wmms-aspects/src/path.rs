use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::error::{AspectError, AspectResult};

#[derive(Clone,Debug,PartialEq,Eq,Hash, PartialOrd, Ord)]
pub struct AspectPath(String);

impl AspectPath {
    pub fn parse(input: &str) -> AspectResult<Self> {
        let s = input.trim();
        if s.is_empty() {
            return Err(AspectError::InvalidPath(input.to_string()));
        }

        let parts: Vec<&str> = s.split('.').collect();
        if parts.iter().any(|part| part.is_empty()) {
            return Err(AspectError::InvalidPath(input.to_string()));
        }

        for p in &parts {
            if !p.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
                return Err(AspectError::InvalidPath(input.to_string()));
            }
        }

        Ok(AspectPath(parts.join(".")))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn parent(&self) -> Option<AspectPath> {
        self.0.rfind('.').map(|idx| AspectPath(self.0[..idx].to_string()))
    }
    
}