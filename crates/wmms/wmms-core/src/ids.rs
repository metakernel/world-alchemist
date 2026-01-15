use std::{fmt::{Display, Formatter}, ops::{ Shr, BitAnd}};

use crate::hash::{Hash128, Hash64, hash_str128, hash_str64};


pub trait IdPrefix{
    const PREFIX: &'static str;
}

#[macro_export]
macro_rules! define_id128{
    ($name:ident, $prefix:expr) => {
        #[repr(transparent)]
        #[derive(Clone,Copy,Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
        pub struct $name($crate::ids::Hash128);

        impl $crate::ids::IdPrefix for $name {
            const PREFIX: &'static str = $prefix;
        }

        impl $name {
            pub fn new(canonical: &str) -> Self {
                $name(Self::from_canonical(Self::PREFIX, canonical))
            }

            fn from_canonical(prefix: &'static str, canonical: &str) -> Hash128 {
                let full_str = format!("{prefix}:{canonical}");
                let hash = hash_str128(&full_str);
                Hash128::from(hash)
            }

            pub fn as_u128(&self) -> u128 {
                self.0.as_u128()
            }
        }

        impl From<u128> for  $name {
            fn from(v: u128) -> Self {
                $name($crate::ids::Hash128::new(v))
            }
    
        }

        impl Shr<u64> for $name {
            type Output = u64;

            fn shr(self, rhs: u64) -> Self::Output {
            (self >> rhs) as u64
            }
        }
    };
}

#[macro_export]
macro_rules! define_id64{
    ($name:ident, $prefix:expr) => {
        #[repr(transparent)]
        #[derive(Clone,Copy,Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
        pub struct $name($crate::ids::Hash64);

        impl $crate::ids::IdPrefix for $name {
            const PREFIX: &'static str = $prefix;
        }

        impl $name {

            pub fn new(canonical: &str) -> Self {
                $name(Self::from_canonical(Self::PREFIX, canonical))
            }

            pub fn from_canonical(prefix: &'static str, canonical: &str) -> Hash64 {
                let full_str = format!("{prefix}:{canonical}");
                let hash = hash_str64(&full_str);
                Hash64::from(hash)
            }

            pub fn as_u64(&self) -> u64 {
                self.0.as_u64()
            }

            pub fn as_u128(&self) -> u128 {
                self.0.as_u64() as u128
            }

            pub fn as_u32(&self) -> u32 {
                self.0.as_u64() as u32
            }

            pub fn as_usize(&self) -> usize {
                self.0.as_u64() as usize
            }
        }

        impl From<u64> for  $name {
            fn from(v: u64) -> Self {
                $name(Hash64::new(v))
            }
    
        }

        impl Shr<u64> for $name {
            type Output = u64;

            fn shr(self, rhs: u64) -> Self::Output {
            (self >> rhs) as u64
            }
        }
    };
}

// WMMS Core ID types
define_id64!(AspectId, "aspect");
define_id64!(TraitId, "trait");
define_id64!(AbilityId, "ability");
define_id64!(EffectId, "effect");
define_id64!(ArchetypeId, "archetype");
define_id64!(SignalId, "signal");
define_id64!(AttrKeyId, "attr");
define_id64!(EntityAuthId,"entity_auth"); // entity “authored” via .alemb path
define_id64!(RelationId,"relation");
define_id64!(EntityInstId,"entity_runtime"); // entity “runtime” ID
define_id64!(EffectInstId,"effect_instance"); // effect instance ID
define_id64!(EntityRid,"entity_rid"); // entity runtime ID

impl EntityInstId {
    #[inline]
      pub fn from_seed_counter(session_seed: u64, counter: u64) -> Self {
        let v = ((session_seed as u64) << 64) | (counter as u64);
        EntityInstId::from(v)
    }

    #[inline]
    pub fn session_seed(&self) -> u64 {
        (*self >> 64) as u64
    }

    #[inline]
    pub fn counter(&self) -> u64 {
        (self.0.as_u64() & 0xFFFFFFFFFFFFFFFF) as u64
    }
}

#[derive(Clone,Copy,Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum EntityId{
    Auth(EntityAuthId),
    Run(EntityInstId),
}

impl EntityId {
    pub fn is_auth(&self) -> bool {
        matches!(self, Self::Auth(_))
    }

    pub fn is_run(&self) -> bool {
        matches!(self, Self::Run(_))
    }
}

impl From<EntityAuthId> for EntityId{
    fn from(v: EntityAuthId) -> Self {
        EntityId::Auth(v)
    }
}

impl From<EntityInstId> for EntityId{
    fn from(v: EntityInstId) -> Self {
        EntityId::Run(v)
    }
}

impl Display for EntityId{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityId::Auth(a) => write!(f, "Auth({:?})", a),
            EntityId::Run(r) => write!(f, "Run({:016x})", r.as_u64()),
        }
    }
}



