pub mod ds_widget;
mod ds_if;
mod ds_iter;


pub use ds_widget::DsWidget;
use crate::ds_node::ds_if::DsIf;
use crate::ds_node::ds_iter::DsIter;

enum DsTree {
    Widget(Box<DsWidget>),
    If(Box<DsIf>),
    Iter(Box<DsIter>),
}

