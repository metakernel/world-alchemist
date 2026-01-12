use std::collections::BTreeMap;
use wmms_aspects::set::AspectSet;
use wmms_core::{ids::{ArchetypeId, AttrKeyId, EffectId, TraitId}, time::Tick};

use crate::{attr::{AttrStack, EffectInstId}, ids::{EntityId, EntityRid, EntityRunId}};

#[derive(Debug)]
pub struct EntityRecord {
    pub id: EntityId,
    pub rid: EntityRid,
    pub alive: bool,

    pub archetype: Option<ArchetypeId>,
    
    pub traits: Vec<TraitId>,

    pub effects: Vec<EffectInstId>,

    pub aspects: AspectSet,
    pub attrs: EntityAttrs,
}

#[derive(Default,Debug)]
pub struct EntityAttrs {
    pub(crate) stacks: BTreeMap<AttrKeyId, AttrStack>,
}

impl EntityAttrs {
    pub fn stack(&self, key: &AttrKeyId) -> Option<&AttrStack> {
        self.stacks.get(key)
    }

    pub fn stack_mut(&mut self, key: &AttrKeyId) -> Option<&mut AttrStack> {
        self.stacks.get_mut(key)
    }

    pub fn insert(&mut self, key: AttrKeyId, stack: AttrStack) {
        self.stacks.insert(key, stack);
    }
}

#[derive(Clone,Debug)]
pub struct EntityRunIdGen{
    session_seed: u64,
    next_counter: u64,
}

impl EntityRunIdGen {
    pub fn new(session_seed: u64) -> Self {
        Self {session_seed, next_counter: 0 }
    }
    #[inline]
    pub fn alloc(&mut self) -> EntityRunId {
        let id = EntityRunId::from_seed_counter(self.session_seed, self.next_counter);
        self.next_counter += 1;
        id
    }

    pub fn session_seed(&self) -> u64 { self.session_seed}
    pub fn next_counter(&self) -> u64 { self.next_counter }

    pub fn set_next_counter(&mut self, v:u64) { self.next_counter = v;}
}