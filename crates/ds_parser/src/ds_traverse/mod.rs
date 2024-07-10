use crate::ds_node::DsRoot;

pub struct DsTraverse {

}

impl DsTraverse {
    fn traverse(&self, ds_root: DsRoot) {
        ds_root.borrow();
    }
}
