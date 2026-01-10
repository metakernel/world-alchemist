use alloc::string::{String, ToString};
use alloc::vec::Vec;
use wmms_core::canon::CanonicalKey;

use crate::error::{AspectError, AspectResult};

#[derive(Clone,Debug,PartialEq,Eq,Hash, PartialOrd, Ord)]
pub struct AspectPath(CanonicalKey);

impl AspectPath {
    pub fn parse(input: &str) -> AspectResult<Self> {
        Ok(AspectPath(CanonicalKey::from_dotted_ident(input)?))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn key(&self) -> &CanonicalKey {
        &self.0
    }

    pub fn parent(&self) -> Option<AspectPath> {
        let s = self.as_str();
        s.rfind('.').map(|i| {
            AspectPath(CanonicalKey::from_dotted_ident(&s[..i]).expect("validated subset"))
        })
    }
    
}