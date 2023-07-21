use std::cell::RefCell;
use std::rc::Rc;
use proc_macros_inner::DsRef;
use super::DsTreeRef;

#[derive(Debug, DsRef)]
pub struct DsContext {
    pub parent: Option<DsTreeRef>,
    pub tree: DsTreeRef,
}

#[allow(dead_code)]
impl DsContextRef {
    pub fn new(parent: Option<DsTreeRef>, tree: DsTreeRef) -> Self {
        DsContextRef {
            inner: Rc::new(RefCell::new(DsContext {
                parent,
                tree,
            })),
        }
    }
}

#[allow(dead_code)]
impl DsContext {
    pub fn new(parent: Option<DsTreeRef>, tree: DsTreeRef) -> Self {
        DsContext {
            parent,
            tree,
        }
    }
}
