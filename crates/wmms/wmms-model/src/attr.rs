use wmms_core::{ids::{AbilityId, ArchetypeId, EffectId, TraitId}, num::Q16_16, time::Tick};

use crate::ids::EntityId;


#[derive(Clone,PartialEq,Debug)]
pub enum AttrValue{
    Null,
    Bool(bool),
    Int(i64),
    Fixed(Q16_16),
    Float(f32),
    Str(String),
    Entity(EntityId),
    Trait(TraitId),
    Ability(AbilityId),
    Effect(EffectId),
    Archtetype(ArchetypeId),    
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum LayerKind {
    Archetype = 0,
    Trait     = 1,
    Effect    = 2,
    Override  = 3,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum LayerSource {
    Archetype(ArchetypeId),
    Trait(TraitId),
    EffectInstance(EffectInstId),
    Override(u64),
    System(u64),
    
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct EffectInstId(pub u64);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct LayerStamp {
    pub tick: Tick,
    pub seq: u32,
}

#[derive(Clone,Debug)]
pub struct AttrLayer {
    pub kind: LayerKind,
    pub source: LayerSource,
    pub value: AttrValue,
    pub stamp: LayerStamp,
    pub expires_at: Option<Tick>,
    pub priority: i16,
}

#[derive(Default,Clone,Debug)]
pub struct AttrStack {
    layers: Vec<AttrLayer>,
    dirty: bool,
    cached: Option<AttrValue>,

}

impl AttrStack {
    pub fn new() -> Self {
        Self {
            layers: Vec::new(),
            dirty: false,
            cached: None,
        }
    }
    fn sort_layers(&mut self, layer: AttrLayer) {

    }

    pub fn upsert(&mut self, source: LayerSource) {
        
    }

    pub fn remove_by_source(&mut self, source: &LayerSource) {
        
    }

    pub fn purge_expired(&mut self, now: Tick) {
        
    }

    pub fn resolve(&mut self) -> Option<&AttrValue> {
        if self.dirty {
           self.cached = self.layers.last().map(|l| l.value.clone());
           self.dirty = false;
        }
        self.cached.as_ref()
    }

    pub fn explain(&self) -> &[AttrLayer] {
        &self.layers
    }

}