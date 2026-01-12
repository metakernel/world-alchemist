use std::{collections::BTreeMap, sync::Arc};

use wmms_aspects::{registry::{AspectRegistry, AspectRid}, set::AspectSet};
use wmms_core::{ids::{AttrKeyId, TraitId}, time::Tick};

use crate::{attr::{AttrLayer, AttrStack, AttrValue, EffectInstId}, diff::ModelDiff, effect::EffectInstance, entity::EntityRecord, ids::{EntityId, EntityRid}, index::AspectIndex, view::ModelView};

pub struct Model {
    pub aspects_reg: Arc<AspectRegistry>,
    entities: Vec<EntityRecord>,
    by_id: BTreeMap<EntityId, EntityRid>,
    pub aspect_index: AspectIndex,

    effects: Vec<EffectInstance>,
    next_effect_inst: u64,

    pending_diff: ModelDiff,
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
            pending_diff: ModelDiff::default(),
        }
    }

    #[inline]
    fn entity(&self, rid: EntityRid) -> Option<&EntityRecord> {
        self.entities.get(rid.0 as usize)
    }

    #[inline]
    fn entity_mut(&mut self, rid: EntityRid) -> Option<&mut EntityRecord> {
        self.entities.get_mut(rid.0 as usize)
    }

    pub fn spawn_entity(&mut self, id: EntityId) -> EntityRid {
        // Check if entity with this ID already exists
        if let Some(existing) = self.by_id.get(&id).copied() {
            return existing;
        }

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

        self.by_id.insert(id, rid);

        self.pending_diff.spawned.push(rid);
        rid
    }

    pub fn kill_entity(&mut self, rid: EntityRid) {
        let (id, old_aspects) = {
            let Some(entity) = self.entity_mut(rid) else {return;};
            if !entity.alive {
                return;
            }

            entity.alive = false;

            // Take aspects out while we still have the entity mutably borrowed.
            let id = entity.id;
            let old_aspects = core::mem::take(&mut entity.aspects);
            (id, old_aspects)
        };

        // Remove the mapping so that rid_of and has_entity reflect the alive state.
        // (Must happen after the mutable borrow of `self.entities` ends.)
        self.by_id.remove(&id);

        // Removes index entries
        for &a in old_aspects.as_slice().iter() {
            self.aspect_index.remove(rid, a);
        }

        self.pending_diff.killed.push(rid);

    }

    pub fn set_entity_aspects(&mut self, rid: EntityRid, direct: &[AspectRid]) {
        // Build the new set (direct + all ancestors)
        let new_aspects = self.aspects_reg.close_under_ancestors(direct);

        // Take old aspect to avoid overlapping mutable borrows
        let old_aspects = if let Some(entity) = self.entity_mut(rid) {
            if !entity.alive {
                return;
            }
            core::mem::take(&mut entity.aspects)
        } else {
            return;
        };

        // remove aspects that are no longer apply
        for &aspect in old_aspects.as_slice().iter() {
            if !new_aspects.contains(aspect) {
                self.aspect_index.remove(rid, aspect);
            }
        }

        // insert newly added aspects
        for &aspect in new_aspects.as_slice().iter() {
            if !old_aspects.contains(aspect){
                self.aspect_index.insert(rid, aspect);
            }
        }

        let changed = old_aspects != new_aspects;

        // Put the new aspects back on the entity
        if let Some(entity) = self.entity_mut(rid) {
            entity.aspects = new_aspects;
        }

        // diff
        if changed {
            self.pending_diff.aspects_changed.push(rid);
        }
    }

    fn insert_sorted_unique_trait(v: &mut Vec<TraitId>, t: TraitId) -> bool {
        match v.binary_search(&t) {
            Ok(_) => false, // already present
            Err(pos) => {
                v.insert(pos, t);
                true
            }
        }
    }

    fn remove_sorted_trait(v: &mut Vec<TraitId>, t: TraitId) -> bool {
        match v.binary_search(&t) {
            Ok(i) => {
                v.remove(i);
                true
            }
            Err(_) => false,
        }
    }

    pub fn add_trait(&mut self, rid: EntityRid, t: TraitId) {
        let Some(entity) = self.entity_mut(rid) else {return;};
        if !entity.alive {
            return;
        }

        if Self::insert_sorted_unique_trait(&mut entity.traits, t) {
            self.pending_diff.trait_added.push((rid,t));
        }
    }

    pub fn remove_trait(&mut self, rid: EntityRid, t: TraitId) {
        let Some(entity) = self.entity_mut(rid) else {return;};
        if !entity.alive {
            return;
        }

        if Self::remove_sorted_trait(&mut entity.traits, t) {
            self.pending_diff.trait_removed.push((rid,t));
        }
    }

    pub fn upsert_attr_layer(&mut self, rid: EntityRid, key: AttrKeyId, layer: AttrLayer) {
        let Some(entity) = self.entity_mut(rid) else {return;};
        if !entity.alive {
            return;
        }

        if let Some(stack) = entity.attrs.stack_mut(&key) {
            stack.upsert(layer);
        } else {
            let mut stack = AttrStack::new();
            stack.upsert(layer);
            entity.attrs.insert(key, stack);
        }

        self.pending_diff.attr_changed.push((rid, key));
    }

    pub fn finalize_commit(&mut self, now: Tick) {
        for (i, entity) in self.entities.iter_mut().enumerate() {
            if !entity.alive {
                continue;
            }

            let rid = EntityRid(i as u32);

            for (key, stack) in entity.attrs.stacks.iter_mut() {
                stack.purge_expired(now);

                if stack.is_dirty() {
                    self.pending_diff.attr_changed.push((rid, *key));
                    let _ = stack.resolve();
                }
            }
        }
    }

    // Effects instances
    pub fn alloc_effect_inst(&mut self) -> EffectInstId {
        let id = EffectInstId(self.next_effect_inst);
        self.next_effect_inst += 1;
        id
    }

    pub fn insert_effect_instance(&mut self, inst: EffectInstance) {
        let inst_id = inst.inst_id;
        let owner = inst.owner;

        let existed = match self.effects.binary_search_by_key(&inst_id, |e| e.inst_id) {
            Ok(pos) => { self.effects[pos] = inst; true},
            Err(pos) => { self.effects.insert(pos, inst); false},
        };
        if !existed{
            if let Some(ent) = self.entity_mut(owner) {
                if ent.alive {
                    if ent.effects.binary_search(&inst_id).is_err() {
                        let pos = ent.effects.binary_search(&inst_id).err().unwrap_or_default();
                        ent.effects.insert(pos, inst_id);
                    }
                }
            }
        }
    }

    pub fn remove_effect_instance(&mut self, inst_id: EffectInstId) {
        let owner = match self.effects.binary_search_by_key(&inst_id, |e| e.inst_id) {
            Ok(pos) => {
                let owner = self.effects[pos].owner;
                self.effects.remove(pos);
                Some(owner)
            }
            Err(_) => None,
        };

        if owner.is_some() {
            self.pending_diff.effect_removed.push(inst_id);
        }

        if let Some(owner) = owner {
            if let Some(ent) = self.entity_mut(owner) {
                if ent.alive {
                    if let Ok(pos) = ent.effects.binary_search(&inst_id) {
                        ent.effects.remove(pos);
                    }
                }
            }
        }
    }

    pub fn take_diff(&mut self) -> ModelDiff {

        self.pending_diff.canonicalize();

        core::mem::take(&mut self.pending_diff)
    }
}

impl ModelView for Model {
    fn has_entity(&self, id: EntityId) -> bool {
        let Some(rid) = self.by_id.get(&id).copied() else {return false;};
        self.entity(rid).map(|e| e.alive).unwrap_or(false)
    }

    fn rid_of(&self, id: EntityId) -> Option<EntityRid> {
        let rid = *self.by_id.get(&id)?;
        let e = self.entity(rid)?;
        if e.alive {Some(rid)} else {None}
    }

    fn id_of(&self, rid: EntityRid) -> Option<EntityId> {
        self.entity(rid).map(|e| e.id)
    }

    fn aspects(&self, rid: EntityRid) -> &AspectSet {
        &self.entity(rid).expect("invalid EntityRid").aspects
    }

    fn matches(&self, rid: EntityRid, q: &wmms_aspects::query::AspectQuery) -> bool {
        let entity = match self.entity(rid) {
            Some(e) if e.alive => e,
            _ => return false,
        };
        q.matches(&entity.aspects)
    }

    fn get_attr(&self, rid: EntityRid, key: AttrKeyId) -> Option<&AttrValue> {
        let entity = self.entity(rid)?;
        if !entity.alive {
            return None;
        }
        let stack = entity.attrs.stack(&key)?;
        stack.cached()
    }

    fn explain_attr(&self, rid: EntityRid, key: AttrKeyId) -> Option<&[AttrLayer]> {
        let entity = self.entity(rid)?;
        if !entity.alive {
            return None;
        }

        let stack = entity.attrs.stack(&key)?;
        Some(stack.layers())
    }

    fn has_trait(&self, rid: EntityRid, t: TraitId) -> bool {
        let entity = match self.entity(rid) {
            Some(e) if e.alive => e,
            _ => return false,
        };
        entity.traits.binary_search(&t).is_ok()
    }
}