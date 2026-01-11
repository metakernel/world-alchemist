use std::{collections::BTreeMap, sync::Arc};

use wmms_aspects::registry::{AspectRegistry, AspectRid};
use wmms_core::{ids::{AttrKeyId, TraitId}, time::Tick};

use crate::{attr::{AttrLayer, AttrStack, EffectInstId}, diff::ModelDiff, effect::EffectInstance, entity::EntityRecord, ids::{EntityId, EntityRid}, index::AspectIndex};

pub struct Model {
    pub aspects_reg: Arc<AspectRegistry>,
    entities: Vec<EntityRecord>,
    by_id: BTreeMap<EntityRid, EntityRid>,
    pub aspect_index: AspectIndex,

    effects: Vec<EffectInstance>,
    next_effect_inst: u64,
}

impl Model {
    pub fn new(aspects_reg: Arc<AspectRegistry>) -> Self {
        let num_aspects = aspects_reg.len();
        Self {
            aspects_reg,
            entities: Vec::new(),
            by_id: BTreeMap::new(),
            aspect_index: AspectIndex::new(num_aspects),
            effects: Vec::new(),
            next_effect_inst: 0,
        }
    }

    pub fn spawn_entity(&mut self, id: EntityId) -> EntityRid {
        let rid = EntityRid(self.entities.len() as u32);
        let record = EntityRecord {
            id,
            rid,
            alive: true,
            archetype: None,
            traits: Vec::new(),
            effects: Vec::new(),
            aspects: wmms_aspects::set::AspectSet::default(),
            attrs: Default::default(),
        };
        self.entities.push(record);
        self.by_id.insert(rid, rid);
        rid
    }

    pub fn kill_entity(&mut self, rid: EntityRid) {
        if let Some(entity) = self.entities.get_mut(rid.0 as usize) {
            entity.alive = false;
        }
    }

    pub fn set_entity_aspects(&mut self, rid: EntityRid, direct: &[AspectRid]) {
        // Build the new set (direct + all ancestors)
        let new_aspects = self.aspects_reg.close_under_ancestors(direct);

        // Temporarily take the old aspects out to avoid overlapping mutable borrows
        // of `self.entities` and `self.aspect_index`.
        let old_aspects = if let Some(entity) = self.entities.get_mut(rid.0 as usize) {
            core::mem::take(&mut entity.aspects)
        } else {
            return;
        };

        // Update aspect index: remove aspects that no longer apply
        for &aspect in old_aspects.as_slice().iter() {
            if !new_aspects.contains(aspect) {
                self.aspect_index.remove(rid, aspect);
            }
        }

        // Update aspect index: insert newly added aspects
        for &aspect in new_aspects.as_slice().iter() {
            if !old_aspects.contains(aspect) {
                self.aspect_index.insert(rid, aspect);
            }
        }

        // Put the new aspects back on the entity
        if let Some(entity) = self.entities.get_mut(rid.0 as usize) {
            entity.aspects = new_aspects;
        }
    }

    pub fn add_trait(&mut self, rid: EntityRid, t: TraitId) {
        if let Some(entity) = self.entities.get_mut(rid.0 as usize) {
            if !entity.traits.contains(&t) {
                entity.traits.push(t);
            }
        }
    }

    pub fn remove_trait(&mut self, rid: EntityRid, t: TraitId) {
        if let Some(entity) = self.entities.get_mut(rid.0 as usize) {
            entity.traits.retain(|&tr| tr != t);
        }
    }

    pub fn upsert_attr_layer(&mut self, rid: EntityRid, key: AttrKeyId, layer: AttrLayer) {
        if let Some(entity) = self.entities.get_mut(rid.0 as usize) {
            if let Some(stack) = entity.attrs.stack_mut(&key) {
                stack.upsert(layer.source);
            } else {
                let mut stack = AttrStack::new();
                stack.upsert(layer.source);
                entity.attrs.insert(key, stack);
            }
        }
    }

    pub fn purge_expired_layer(&mut self, tick: Tick) {
        for entity in &mut self.entities {
            for stack in entity.attrs.stacks.values_mut() {
                stack.purge_expired(tick);
            }
        }
    }

    // Effects instances
    pub fn alloc_effect_inst(&mut self) -> EffectInstId {
        let id = EffectInstId(self.next_effect_inst);
        self.next_effect_inst += 1;
        id
    }

    pub fn insert_effect_instance(&mut self,inst: EffectInstance) {
        self.effects.push(inst);
    }

    pub fn remove_effect_instance(&mut self, inst_id: EffectInstId) {
        self.effects.retain(|e| e.inst_id != inst_id);
    }

    pub fn take_diff(&mut self) -> ModelDiff {
        ModelDiff::default()
    }
}