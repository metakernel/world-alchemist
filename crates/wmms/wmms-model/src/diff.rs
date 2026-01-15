use wmms_core::ids::{AttrKeyId, TraitId};

use wmms_core::{ids::EffectInstId, ids::EntityRid};

#[derive(Debug,Clone,Default)]
pub struct ModelDiff {
    pub spawned: Vec<EntityRid>,
    pub killed: Vec<EntityRid>,

    pub trait_added: Vec<(EntityRid,TraitId)>,
    pub trait_removed: Vec<(EntityRid,TraitId)>,

    pub attr_changed: Vec<(EntityRid, AttrKeyId)>,

    pub effect_added: Vec<EffectInstId>,
    pub effect_removed: Vec<EffectInstId>,

    pub aspects_changed: Vec<EntityRid>,
    
}
impl ModelDiff {

    fn sort_dedup<T: Ord>(v: &mut Vec<T>) {
        v.sort();
        v.dedup();
    }

    pub fn canonicalize(&mut self) {
        Self::sort_dedup(&mut self.spawned);
        Self::sort_dedup(&mut self.killed);
        Self::sort_dedup(&mut self.trait_added);
        Self::sort_dedup(&mut self.trait_removed);
        Self::sort_dedup(&mut self.attr_changed);
        Self::sort_dedup(&mut self.effect_added);
        Self::sort_dedup(&mut self.effect_removed);
        Self::sort_dedup(&mut self.aspects_changed);
    }
}