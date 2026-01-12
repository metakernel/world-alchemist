use core::fmt::Display;
use std::fmt::{Formatter};

use wmms_core::ids::EntityAuthId;

#[repr(transparent)]
#[derive(Clone,Copy,Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct EntityRid(pub u32);

#[repr(transparent)]
#[derive(Clone,Copy,Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct EntityRunId(pub u128);

#[derive(Clone,Copy,Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum EntityId{
    Auth(EntityAuthId),
    Run(EntityRunId),
}

impl EntityRunId{
    #[inline]
    pub fn from_seed_counter(session_seed: u64, counter: u64) -> Self {
        let v = ((session_seed as u128) << 64) | (counter as u128);
        EntityRunId(v)
    }

    #[inline]
    pub fn session_seed(&self) -> u64 {
        (self.0 >> 64) as u64
    }

    #[inline]
    pub fn counter(&self) -> u64 {
        (self.0 & 0xFFFFFFFFFFFFFFFF) as u64
    }
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

impl From<EntityRunId> for EntityId{
    fn from(v: EntityRunId) -> Self {
        EntityId::Run(v)
    }
}

impl Display for EntityId{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityId::Auth(a) => write!(f, "Auth({:?})", a),
            EntityId::Run(r) => write!(f, "Run({:032x})", r.0),
        }
    }
}
