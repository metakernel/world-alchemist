pub mod error;
pub mod hash;
pub mod ids;
pub mod canon;
pub mod time;
pub mod num;
pub mod rng;

pub mod prelude {
    pub use crate::error::{Result, WMMSCoreError};
    pub use crate::ids::*;
    pub use crate::hash::{Hash128, Hash64};
    pub use crate::canon::{CanonicalKey, CanonMap, CanonSet, canon_sort};
    pub use crate::time::{Tick,TickDelta,TickRate};
    pub use crate::num::{FixedU32, Q16_16, Q24_8, quantize_f32};
    pub use crate::rng::DetRng;
}
