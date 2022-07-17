use crate::prelude::*;

pub trait Clear {
    fn cls_all();
    fn clear_all();
}

impl Clear for BTerm {
    fn cls_all() {
        BACKEND_INTERNAL
            .lock()
            .consoles
            .iter_mut()
            .for_each(|c| c.console.cls());
    }

    fn clear_all() {
        BACKEND_INTERNAL.lock().consoles.clear();
    }
}
