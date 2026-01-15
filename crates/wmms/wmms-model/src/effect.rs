use wmms_core::{ids::{EffectId, EffectInstId, EntityRid}, time::Tick};

#[derive(Clone,Debug)]
pub struct EffectInstance {
    pub inst_id: EffectInstId,
    pub effect_id: EffectId,
    pub owner: EntityRid,
    pub source: Option<EntityRid>,
    pub stack_key: u64,
    pub applied_at: Tick,
    pub expires_at: Option<Tick>,
}