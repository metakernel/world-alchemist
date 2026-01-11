use wmms_core::ids::{Id128, RelationId};

use crate::ids::EntityRunId;

pub struct Relation {
    pub out_edges: Vec<Vec<(RelationId,EntityRunId)>>,
}