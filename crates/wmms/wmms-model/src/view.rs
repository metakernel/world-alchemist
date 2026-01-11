use wmms_aspects::{query::AspectQuery, set::AspectSet};
use wmms_core::ids::{AttrKeyId, TraitId};

use crate::{attr::{AttrLayer, AttrValue}, ids::{EntityId, EntityRid}};

pub trait ModelView {
    fn has_entity(&self, id: EntityId) -> bool;
    fn rid_of(&self, id: EntityId) -> Option<EntityRid>;
    fn id_of(&self, rid: EntityRid) -> Option<EntityId>;

    fn aspects(&self, rid: EntityRid) -> &AspectSet;
    fn matches(&self, rid: EntityRid, q: &AspectQuery) -> bool;

    fn get_attr(&self, key: AttrKeyId) -> Option<&AttrValue>;
    fn explain_attr(&self, rid: EntityRid, key: AttrKeyId) -> Option<&[AttrLayer]>;

    fn has_trait(&self, rid: EntityRid, t: TraitId) -> bool;
}