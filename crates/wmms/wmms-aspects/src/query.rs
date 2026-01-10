use alloc::vec::Vec;
use crate::registry::AspectRid;
use crate::set::AspectSet;

#[derive(Clone,Debug,Default)]
pub struct AspectQuery{
    pub all_of: Vec<AspectRid>,
    pub any_of: Vec<AspectRid>,
    pub none_of: Vec<AspectRid>,
}

impl AspectQuery{
    pub fn normalize(mut self) -> Self {
        self.all_of.sort();
        self.all_of.dedup();
        self.any_of.sort();
        self.any_of.dedup();
        self.none_of.sort();
        self.none_of.dedup();
        self

    }

    pub fn matches(&self, aspects: &AspectSet) -> bool {
        // all_of: must contain all
        for a in &self.all_of{
            if !aspects.contains(*a){
                return false;
            }
        }
        // none_of: must contain none
        for n in &self.none_of{
            if aspects.contains(*n){
                return false;
            }
        }
        // any_of: if empty -> ok, else at least one
        if self.any_of.is_empty(){
            return true;
        }

        for y in &self.any_of{
            if aspects.contains(*y){
                return true;
            }
        }

        false
    }
}