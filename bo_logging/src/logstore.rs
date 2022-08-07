use super::*;
use lazy_static::lazy_static;
use parking_lot::Mutex;

lazy_static! {
    static ref LOG: Mutex<Vec<Vec<LogFragment>>> = Mutex::new(Vec::new());
}

pub fn append_entry(fragments: Vec<LogFragment>) { LOG.lock().push(fragments); }

pub fn clear_log() { LOG.lock().clear(); }

pub fn print_log(console: usize, pos: Point) {
    let mut batch = DrawBatch::new();
    batch.target(console);

    let mut y = pos.y;
    let mut x = pos.x;
    LOG.lock().iter().rev().take(6).for_each(|log| {
        log.iter().for_each(|frag| {
            batch.print_color(Point::new(x, y), &frag.text, ColorPair::new(frag.color.to_rgba(1.0), BLACK));
            x += frag.text.len() as i32;
            x += 1;
        });
        y += 1;
        x = pos.x;
    });

    batch.submit(5000).expect("Batch error");
}

pub fn clone_log() -> Vec<Vec<LogFragment>> { LOG.lock().clone() }

pub fn restore_log(log: &mut Vec<Vec<LogFragment>>) {
    LOG.lock().clear();
    LOG.lock().append(log);
}
