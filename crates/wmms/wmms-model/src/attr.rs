use wmms_core::{ids::{AbilityId, ArchetypeId, EffectId, EffectInstId, TraitId}, num::Q16_16, time::Tick};

use wmms_core::ids::EntityId;


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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct LayerStamp {
    pub tick: Tick,
    pub seq: u32,
}

#[derive(Clone,Debug, PartialEq)]
pub struct AttrLayer {
    pub kind: LayerKind,
    pub source: LayerSource,
    pub value: AttrValue,
    pub stamp: LayerStamp,
    pub expires_at: Option<Tick>,
    pub priority: i16,
}

impl AttrLayer {
    #[inline]
    fn order_key(&self) -> (LayerKind, i16, LayerSource, Tick, u32) {
        (self.kind, self.priority, self.source, self.stamp.tick, self.stamp.seq)
    }
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

    #[inline]
    pub fn is_dirty(&self) -> bool {self.dirty}

    #[inline]
    pub fn cached(&self) -> Option<&AttrValue> {self.cached.as_ref()}

    #[inline]
    pub fn layers(&self) -> &[AttrLayer] { &self.layers}

    // Ensure layers are sorted by priority and stamp
    fn sort_layers(&mut self) {
        self.layers.sort_by(|a,b| a.order_key().cmp(&b.order_key()));

        let mut i = 0usize;
        while i + 1 < self.layers.len() {
            let same_key = self.layers[i].kind == self.layers[i+1].kind
                && self.layers[i].source == self.layers[i + 1].source;
            if same_key {
                // after sorting, the stronger one is the one with the larger order_key
                let keep_next = self.layers[i].order_key() <= self.layers[i+1].order_key();
                if keep_next {
                    self.layers.remove(i); // drop weaker
                } else {
                    self.layers.remove(i + 1);
                }
                continue; // re-check at same index
                }
            }
            i += 1;
        }

    // Upsert a layer based on its source
    pub fn upsert(&mut self, layer: AttrLayer) {
        if let Some(idx) = self
        .layers
        .iter()
        .position(|l| l.kind == layer.kind && l.source == layer.source) {
            self.layers[idx] = layer;
        } else {
            self.layers.push(layer);
        }
        self.dirty = true;
        self.sort_layers();
    }

    pub fn remove_by_source(&mut self, source: LayerSource) {
        let before = self.layers.len();
        self.layers.retain(|l| l.source != source);
        if self.layers.len() != before {
            self.dirty = true;
        }
        
    }

    pub fn purge_expired(&mut self, now: Tick) {
        let before = self.layers.len();
        self.layers.retain(|l| match l.expires_at {
            Some(expiry) => expiry > now, // expire if expiry <= now
            None => true,
        });
        if self.layers.len() != before {
            self.dirty = true;
        }
    }

    /// Resolve cached winner if dirty. Winner = last layer (strongest).
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