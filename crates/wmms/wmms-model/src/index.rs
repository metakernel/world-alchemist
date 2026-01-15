use roaring::RoaringBitmap;
use wmms_aspects::registry::AspectRid;

use wmms_core::ids::EntityRid;

pub struct AspectIndex {
    by_aspect: Vec<RoaringBitmap>,
}

impl AspectIndex {
    pub fn new(num_aspects: usize) -> Self {
        Self {by_aspect: (0..num_aspects).map(|_| RoaringBitmap::new()).collect() }
    }

    pub fn insert(&mut self, entity: EntityRid, aspect: AspectRid) {
        if let Some(bitmap) = self.by_aspect.get_mut(aspect.0 as usize) {
            bitmap.insert(entity.as_u32());
        }
    }

    pub fn remove(&mut self, entity: EntityRid, aspect: AspectRid) {
        if let Some(bitmap) = self.by_aspect.get_mut(aspect.0 as usize) {
            bitmap.remove(entity.as_u32());
        }
    }

    pub fn has_aspect(&self, entity: EntityRid, aspect: AspectRid) -> bool {
        if let Some(bitmap) = self.by_aspect.get(aspect.0 as usize) {
            bitmap.contains(entity.as_u32())
        } else {
            false
        }
    }

    pub fn bitmap(&self, aspect: AspectRid) -> Option<&RoaringBitmap> {
        self.by_aspect.get(aspect.0 as usize)
    }
}