use std::collections::BTreeMap;
use wmms_aspects::set::AspectSet;
use wmms_core::{ids::{ArchetypeId, AttrKeyId, EffectId, TraitId}, time::Tick};

use crate::{attr::{AttrStack, EffectInstId}, ids::{EntityId, EntityRid}};

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