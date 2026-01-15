
use wmms_aspects::registry::AspectRid;
use wmms_core::{ids::{AttrKeyId, EffectId, EffectInstId, EntityRid, TraitId}, time::Tick};

use crate::{attr::{AttrLayer, AttrValue, LayerKind, LayerSource, LayerStamp}, effect::EffectInstance, model::Model};

#[derive(Clone, Debug)]
pub struct AttrLayerSpec {
    pub kind: LayerKind,
    pub source: LayerSource,
    pub value: AttrValue,
    pub expires_at: Option<Tick>,
    pub priority: i16,
}

impl AttrLayerSpec {
    #[inline]
    pub fn into_layer(self, tick: Tick, seq: u32) -> AttrLayer {
        AttrLayer {
            kind: self.kind,
            source: self.source,
            value: self.value,
            stamp: LayerStamp { tick, seq },
            expires_at: self.expires_at,
            priority: self.priority,
        }
    }
}

#[derive(Clone, Debug)]
pub struct EffectSpec {
    pub effect_id: EffectId,
    pub owner: EntityRid,
    pub source: Option<EntityRid>,
    pub stack_key: u64,
    pub expires_at: Option<Tick>,
}

#[derive(Clone, Debug)]
pub enum EffectOp {
    //  ----- Traits -----
    AddTrait {
        target: EntityRid,
        trait_id: TraitId,
    },
    RemoveTrait {
        target: EntityRid,
        trait_id: TraitId,
    },

    // -----Attributes (layered) -----
    UpsertAttrLayer {
        target: EntityRid,
        key: AttrKeyId,
        layer: AttrLayerSpec,
    },

    RemoveAttrLayersBySource {
        target: EntityRid,
        source: LayerSource,
    },

    // ----- Aspects -----
    SetAspectsDirect {
        target: EntityRid,
        aspects: Vec<AspectRid>,
    },

    // ----- Effects (Instances) -----
    ApplyEffect {
        spec: EffectSpec,
    },

    RemoveEffect {
        inst_id: EffectInstId,
    },

    // ---- Entity Lifecycle -----
    KillEntity {
        target: EntityRid,
    },

}

pub struct ApplyCtx {
    pub now: Tick,
    pub seq: u32,
}

impl ApplyCtx {
    #[inline]
    fn next_seq(&mut self) -> u32 {
        let v = self.seq;
        self.seq += 1;
        v
    }
}

pub fn apply_ops(model: &mut Model, ctx: &mut ApplyCtx, ops: &[EffectOp]) {
    for op in ops {
        match op {
            EffectOp::AddTrait { target, trait_id } => {
                model.add_trait(*target, *trait_id);
            }
            EffectOp::RemoveTrait { target, trait_id } => {
                model.remove_trait(*target, *trait_id);
            }
            EffectOp::UpsertAttrLayer { target, key, layer } => {
                let layer = layer.clone().into_layer(ctx.now, ctx.next_seq());
                model.upsert_attr_layer(*target, *key, layer);
            }
            EffectOp::RemoveAttrLayersBySource { target, source } => {
                if let Some(ent) = model.entity_mut(*target){
                    for (_k, stack) in ent.attrs.stacks.iter_mut(){
                        stack.remove_by_source(*source);
                    }
                }
            }
            EffectOp::SetAspectsDirect { target, aspects } => {
                model.set_entity_aspects(*target, aspects.as_slice());
            }
            EffectOp::ApplyEffect { spec } => {
               let inst_id = model.alloc_effect_inst();
                let inst = EffectInstance {
                    inst_id,
                    effect_id: spec.effect_id,
                    owner: spec.owner,
                    source: spec.source,
                    stack_key: spec.stack_key,
                    applied_at: ctx.now,
                    expires_at: spec.expires_at,
                };
            model.insert_effect_instance(inst);
            }
            EffectOp::RemoveEffect { inst_id } => {
                model.remove_effect_instance(*inst_id);
            }
            EffectOp::KillEntity { target } => {
                model.kill_entity(*target);
            }
        }
    }
}