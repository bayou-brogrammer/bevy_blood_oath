use super::*;
use parking_lot::Mutex;

lazy_static! {
    static ref LOG: Mutex<Vec<Vec<LogFragment>>> = Mutex::new(Vec::new());
}

pub fn append_entry(fragments: Vec<LogFragment>) {
    LOG.lock().push(fragments);
}

pub fn clear_log() {
    LOG.lock().clear();
}

pub fn print_log(console: &mut Box<dyn Console>, pos: Point) {
    let mut y = pos.y;
    let mut x = pos.x;
    LOG.lock().iter().rev().take(6).for_each(|log| {
        log.iter().for_each(|frag| {
            console.print_color(
                x,
                y,
                frag.color.to_rgba(1.0),
                RGBA::named(BLACK),
                &frag.text,
            );
            x += frag.text.len() as i32;
            x += 1;
        });
        y += 1;
        x = pos.x;
    });
}

pub fn clone_log() -> Vec<Vec<crate::gamelog::LogFragment>> {
    LOG.lock().clone()
}

pub fn restore_log(log: &mut Vec<Vec<crate::gamelog::LogFragment>>) {
    LOG.lock().clear();
    LOG.lock().append(log);
}
