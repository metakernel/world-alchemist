pub mod error;
pub mod hash;
pub mod ids;
pub mod canon;
pub mod time;
pub mod num;

pub mod prelude {
    pub use crate::error::{Result, WMMSCoreError};
    pub use crate::ids::*;
    pub use crate::canon::{CanonicalKey, CanonMap, CanonSet, canon_sort};
    pub use crate::time::*;
}
