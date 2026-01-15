use wmms_core::ids::{ RelationId};

use wmms_core::ids::EntityInstId;

pub struct Relation {
    pub out_edges: Vec<Vec<(RelationId,EntityInstId)>>,
}