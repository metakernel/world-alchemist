#![no_std]
extern crate alloc;

pub mod error;
pub mod path;
pub mod registry;
pub mod set;
pub mod query;

#[cfg(test)]
mod tests{
    use alloc::vec;

    use super::*;
    use crate::{registry::AspectRegistryBuilder, set::AspectSet};

    #[test]
    fn registry_registers_ancestors_and_is_deterministic() {
        let mut b = AspectRegistryBuilder::new();
        b.register("damage.fire").unwrap();
        b.register("damage.ice").unwrap();

        let r = b.seal().unwrap();

        let damage = r.resolve_path("damage").unwrap();
        let fire = r.resolve_path("damage.fire").unwrap();
        let ice  = r.resolve_path("damage.ice").unwrap();

        assert!(r.is_descendant_of(fire, damage));
        assert!(r.is_descendant_of(ice, damage));
        assert_eq!(r.parent(damage), None);
    }

    #[test]
    fn query_matches() {
        let mut b = AspectRegistryBuilder::new();
        b.register("damage.fire").unwrap();
        b.register("status.burning").unwrap();
        let r = b.seal().unwrap();

        let fire = r.resolve_path("damage.fire").unwrap();
        let burning = r.resolve_path("status.burning").unwrap();

        let aspects = AspectSet::from_unsorted(vec![fire, burning]);

        let q = crate::query::AspectQuery {
            all_of: vec![fire],
            any_of: vec![],
            none_of: vec![],
        }.normalize();

        assert!(q.matches(&aspects));
    }
}

