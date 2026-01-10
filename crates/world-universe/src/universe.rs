
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Universe {
    pub name: String,
    pub authors: Option<Vec<String>>,

}

impl Universe {
    pub fn new(name: &str, authors: Option<Vec<String>>) -> Self {
        Self {
            name: name.to_string(),
            authors,
        }
    }
}