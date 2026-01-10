use crate::hash::{Hash128, Hash64, hash_str128};

#[repr(transparent)]
#[derive(Clone,Copy,Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct Id128(Hash128);

pub trait IdPrefix{
    const PREFIX: &'static str;
}

pub fn derive_id128(prefix: &'static str, canonical: &str) -> Id128 {
    let full_str = format!("{prefix}:{canonical}");
    let hash = hash_str128(&full_str);
    Id128(hash)
}

#[macro_export]
macro_rules! define_id{
    ($name:ident, $prefix:expr) => {
        #[repr(transparent)]
        #[derive(Clone,Copy,Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
        pub struct $name($crate::ids::Id128);

        impl $crate::ids::IdPrefix for $name {
            const PREFIX: &'static str = $prefix;
        }

        impl $name {
            pub fn derive(canonical: &str) -> Self {
                let id128 = crate::ids::derive_id128(Self::PREFIX, canonical);
                Self(id128)
            }
        }
    };
}

// WMMS Core ID types
define_id!(AspectId, "aspect");
define_id!(TraitId, "trait");
define_id!(AbilityId, "ability");
define_id!(EffectId, "effect");
define_id!(ArchetypeId, "archetype");
define_id!(SignalId, "signal");
define_id!(AttrKeyId, "attr");
define_id!(EntityAuthId,"entity"); // entity “authored” via .alemb path

