use alloc::vec::Vec;
use crate::registry::{AspectRid};

#[derive(Clone,Debug,Default, PartialEq)]
pub struct AspectSet{
    rids: Vec<AspectRid>,
}

impl AspectSet{
    pub fn new()->Self{
        Self{
            rids: Vec::new(),
        }
    }

    pub fn from_unsorted(mut v: Vec<AspectRid>) -> Self {
        v.sort();
        v.dedup();
        Self{
            rids: v,
        }
    }

    pub fn as_slice(&self) -> &[AspectRid] {
        &self.rids
    }

    pub fn insert(&mut self, rid: AspectRid){
        match self.rids.binary_search(&rid){
            Ok(_)=>{}, // already present
            Err(pos)=>self.rids.insert(pos,rid),
        }
    }
    pub fn contains(&self, rid: AspectRid) -> bool {
        self.rids.binary_search(&rid).is_ok()
    }
}
    