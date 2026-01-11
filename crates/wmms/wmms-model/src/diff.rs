use wmms_core::ids::{AttrKeyId, TraitId};

use crate::{attr::EffectInstId, ids::EntityRid};

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

impl Default for ModelDiff {
    fn default() -> Self {
        Self {
            spawned: Vec::new(),
            killed: Vec::new(),
            trait_added: Vec::new(),
            trait_removed: Vec::new(),
            attr_changed: Vec::new(),
            effect_added: Vec::new(),
            effect_removed: Vec::new(),
            aspects_changed: Vec::new(),
        }
    }
}