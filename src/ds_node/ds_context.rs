use std::cell::RefCell;
use std::rc::Rc;
use super::DsTreeRef;

pub struct DsContext {
    pub parent: Option<DsTreeRef>,
    pub tree: DsTreeRef,
}

pub struct DsContextRef {
    inner: Rc<RefCell<DsContext>>,
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

    pub fn borrow(&self) -> std::cell::Ref<DsContext> {
        self.inner.borrow()
    }

    pub fn borrow_mut(&self) -> std::cell::RefMut<DsContext> {
        self.inner.borrow_mut()
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

    pub fn into_ref(self) -> DsContextRef {
        DsContextRef::new(self.parent, self.tree)
    }
}

impl Clone for DsContextRef {
    fn clone(&self) -> Self {
        DsContextRef {
            inner: self.inner.clone(),
        }
    }
}
