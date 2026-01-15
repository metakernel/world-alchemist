use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::collections::BTreeMap;

use wmms_core::hash::hash_str128;
use wmms_core::prelude::*;

use crate::error::{AspectError, AspectResult};
use crate::path::AspectPath;
use crate::set::AspectSet;

#[repr(transparent)]
#[derive(Clone,Copy,Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct AspectRid(pub u32);

#[derive(Clone,Debug)]
pub struct AspectNode{
    pub rid: AspectRid,
    pub id: AspectId,
    pub key: CanonicalKey,
    pub parent: Option<AspectRid>,
    pub children: Vec<AspectRid>,
    pub depth: u16,
}

pub struct AspectRegistry{
    nodes: Vec<AspectNode>,
    by_key: BTreeMap<CanonicalKey, AspectRid>,
    by_id: BTreeMap<AspectId, AspectRid>,
    pub registry_hash: Hash128,
}

pub struct AspectRegistryBuilder{
    keys: BTreeMap<CanonicalKey, ()>,
    sealed: bool,
}

impl AspectRegistryBuilder{
    pub fn new()->Self{
        Self{
            keys: BTreeMap::new(),
            sealed: false,
        }
    }

    pub fn register(&mut self, path: &str)->AspectResult<()>{
        if self.sealed {
            return Err(AspectError::Sealed);
        }
        let path = AspectPath::parse(path)?.as_str().to_string();

        self.insert_with_ancestors(&path)?;
        Ok(())
    }

    fn insert_with_ancestors(&mut self, canonical: &str) -> AspectResult<()> {
        self.keys.entry(CanonicalKey::from_dotted_ident(canonical)?).or_insert(());
        let mut cur = AspectPath::parse(canonical)?;
        while let Some(parent) = cur.parent() {
            self.keys.entry(CanonicalKey::from_dotted_ident(parent.as_str())?).or_insert(());
            cur = parent;
        }

        Ok(())
    }

    pub fn seal(mut self) -> AspectResult<AspectRegistry>{
        self.sealed = true;

        // Assign RIDs in canonical order by path (BTreeMap order)
        let mut nodes: Vec<AspectNode> = Vec::with_capacity(self.keys.len());
        let mut by_key: BTreeMap<CanonicalKey, AspectRid> = BTreeMap::new();
        let mut by_id: BTreeMap<AspectId, AspectRid> = BTreeMap::new();

        for(i, (key,_)) in self.keys.iter().enumerate() {
            let rid = AspectRid(i as u32);
            let id = AspectId::new(key.as_str()); // Stable hash from canonical path
            nodes.push(AspectNode {
                rid, id,
                key: key.clone(),
                parent: None,
                children: Vec::new(),
                depth: 0,
            });
            by_key.insert(key.clone(), rid);
            by_id.insert(id, rid);
        }

        // Set up parent/child relationships and depths
        for idx in 0..nodes.len() {
            let _key = nodes[idx].key.clone();
            let ap = AspectPath::parse(nodes[idx].key.as_str())?;
            if let Some(parent) = ap.parent(){
                let parent_rid = *by_key.get(parent.key())
                .ok_or_else(|| AspectError::UnknownAspect(parent.as_str().to_string()))?;
                nodes[idx].parent = Some(parent_rid);
            }
        }

        // children lists
        for idx in 0..nodes.len() {
            let (maybe_parent, rid) = (nodes[idx].parent, nodes[idx].rid);
            if let Some(p) = maybe_parent {
                nodes[p.0 as usize].children.push(rid);
            }
        }

        // sort children by canonical order (RIDs are assigned in canonical path order)
        for idx in 0..nodes.len() {
            nodes[idx].children.sort_by_key(|c| c.0);
        }

        // compute depth deterministically (walk parents)
        for idx in 0..nodes.len() {
            let mut d: u16 = 0;
            let mut cur = nodes[idx].parent;
            while let Some(p) = cur {
                d = d.saturating_add(1);
                cur = nodes[p.0 as usize].parent;
            }
            nodes[idx].depth = d;
        }

        // registry hash
        let mut acc = String::new();
        for n in &nodes {
            acc.push_str(&n.key.as_str());
            acc.push('|');
            if let Some(p) = n.parent {
                acc.push_str(&nodes[p.0 as usize].key.as_str());
            }
            acc.push('\n');
        }
        let registry_hash = hash_str128(&acc);
        Ok(AspectRegistry{
            nodes,
            by_key,
            by_id,
            registry_hash,
        })
    }
}

impl AspectRegistry{
    pub fn len(&self) -> usize{
        self.nodes.len()
    }
    pub fn is_empty(&self) -> bool{
        self.nodes.is_empty()
    }
    pub fn resolve_path(&self, path: &str) -> Option<AspectRid>{
        let p = AspectPath::parse(path).ok()?;
        self.by_key.get(p.key()).copied()
    }
    pub fn resolve_id(&self, id: &AspectId) -> Option<AspectRid>{
        self.by_id.get(id).copied()
    }
    pub fn node(&self, rid: AspectRid) -> &AspectNode{
        &self.nodes[rid.0 as usize]
    }
    pub fn key(&self, rid: AspectRid) -> &CanonicalKey{
        &self.nodes[rid.0 as usize].key
    }
    pub fn id(&self, rid: AspectRid) -> AspectId{
        self.nodes[rid.0 as usize].id
    }
    pub fn parent(&self, rid: AspectRid) -> Option<AspectRid>{
        self.nodes[rid.0 as usize].parent
    }
    pub fn children(&self, rid: AspectRid) -> &[AspectRid]{
        &self.nodes[rid.0 as usize].children
    }
    pub fn is_descendant_of(&self, mut rid: AspectRid, ancestor: AspectRid) -> bool{
        while let Some(p) = self.parent(rid){
            if p == ancestor {
                return true;
            }
            rid = p;
        }
        false
    }
    pub fn ancestors(&self, mut rid: AspectRid, out: &mut Vec<AspectRid>){
        out.clear();
        while let Some(p) = self.parent(rid){
            out.push(p);
            rid = p;
        }
    }

    pub fn close_under_ancestors(&self, base: &[AspectRid]) -> AspectSet {
        let mut result = AspectSet::new();
        for &rid in base {
            result.insert(rid);
            let mut current = rid;
            while let Some(parent) = self.parent(current) {
                if result.contains(parent) {
                    break;
                }
                result.insert(parent);
                current = parent;
            }
        }
        result
    }

}