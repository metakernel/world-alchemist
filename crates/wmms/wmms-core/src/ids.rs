use std::{fmt::{Display, Formatter}, ops::{ Shr, BitAnd}};

use crate::hash::{Hash128, Hash64, hash_str128, hash_str64};

#[cfg(feature = "id64")]
pub type IdSize = u64;

#[cfg(feature = "id128")]
pub type IdSize = u128;

#[cfg(feature = "id64")]
pub type IdHash = Hash64;

#[cfg(feature = "id128")]
pub type IdHash = Hash128;


pub trait IdPrefix{
    const PREFIX: &'static str;
}

#[macro_export]
macro_rules! define_id {
    ($name:ident, $prefix:expr) => {
        #[repr(transparent)]
        #[derive(Clone,Copy,Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
        pub struct $name($crate::ids::IdHash);

        impl $crate::ids::IdPrefix for $name {
            const PREFIX: &'static str = $prefix;
        }

        impl $name {
            pub fn new(canonical: &str) -> Self {
                $name(Self::from_canonical(Self::PREFIX, canonical))
            }
            
            fn from_canonical(prefix: &'static str, canonical: &str) -> $crate::ids::IdHash {
                let full_str = format!("{prefix}:{canonical}");

                #[cfg(feature = "id64")]
                let hash = hash_str64(&full_str);

                #[cfg(feature = "id128")]
                let hash = hash_str128(&full_str);

                IdHash::from(hash)
            }

            pub fn as_idsize(&self) -> $crate::ids::IdSize {
                #[cfg(feature = "id64")]
                {
                    self.0.as_u64() as $crate::ids::IdSize
                }
                #[cfg(feature = "id128")]
                {
                    self.0.as_u128() as $crate::ids::IdSize
                }
            }

            pub fn as_u128(&self) -> u128 {
                self.as_idsize() as u128
            }

            pub fn as_u64(&self) -> u64 {
                self.as_idsize() as u64
            }

            pub fn as_u32(&self) -> u32 {
                self.as_idsize() as u32
            }

            pub fn as_usize(&self) -> usize {
                self.as_idsize() as usize
            }
        }

        impl From<u128> for  $name {
            fn from(v: u128) -> Self {
                $name($crate::ids::IdHash::new(v as IdSize))
            }
    
        }

        impl From<u64> for  $name {
            fn from(v: u64) -> Self {
                $name($crate::ids::IdHash::new(v as IdSize))
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
define_id!(AspectId, "aspect");
define_id!(TraitId, "trait");
define_id!(AbilityId, "ability");
define_id!(EffectId, "effect");
define_id!(ArchetypeId, "archetype");
define_id!(SignalId, "signal");
define_id!(AttrKeyId, "attr");
define_id!(EntityAuthId,"entity_auth"); // entity “authored” via .alemb path
define_id!(RelationId,"relation");
define_id!(EntityInstId,"entity_runtime"); // entity “runtime” ID
define_id!(EffectInstId,"effect_instance"); // effect instance ID
define_id!(EntityRid,"entity_rid"); // entity runtime ID

impl EntityInstId {
    #[inline]
      pub fn from_seed_counter(session_seed: u64, counter: u64) -> Self {
        let v = ((session_seed as IdSize) << 64) | (counter as IdSize);
        EntityInstId::from(v)
    }

    #[inline]
    pub fn session_seed(&self) -> u64 {
        (*self >> 64) as u64
    }

    #[inline]
    #[cfg(feature = "id64")]
    pub fn counter(&self) -> u64 {
        (self.0.as_u64() & 0xFFFFFFFFFFFFFFFF) as u64
    }
    #[inline]
    #[cfg(feature = "id128")]
    pub fn counter(&self) -> u128 {
        (self.0.as_u128() & 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF) as u128
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
            #[cfg(feature = "id128")]
            EntityId::Run(r) => write!(f, "Run({:032x})", r.as_u128()),
            #[cfg(feature = "id64")]
            EntityId::Run(r) => write!(f, "Run({:016x})", r.as_u64()),
        }
    }
}



