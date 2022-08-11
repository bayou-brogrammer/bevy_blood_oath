use crate::prelude::*;

pub trait BTermAdditions {
    fn reset_consoles();
    fn clear_all_internal_consoles();
    fn clear_consoles(&mut self, consoles: &[usize]);
    fn get_key(&self) -> Option<GameKey>;
}

impl BTermAdditions for BTerm {
    fn reset_consoles() { BACKEND_INTERNAL.lock().consoles.clear(); }

    fn clear_all_internal_consoles() {
        BACKEND_INTERNAL.lock().consoles.iter_mut().for_each(|c| c.console.cls());
    }

    fn clear_consoles(&mut self, consoles: &[usize]) {
        for layer in consoles.iter() {
            self.set_active_console(*layer);
            self.cls();
        }

        if !consoles.is_empty() {
            self.set_active_console(consoles[0])
        }
    }

    fn get_key(&self) -> Option<GameKey> { self.key.get_key() }
}
