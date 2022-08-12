use super::*;
use lazy_static::lazy_static;
use parking_lot::Mutex;

lazy_static! {
    static ref LOG: Mutex<Vec<Vec<LogFragment>>> = Mutex::new(Vec::new());
}

pub fn clear_log() { LOG.lock().clear(); }
pub fn append_entry(fragments: Vec<LogFragment>) { LOG.lock().push(fragments); }

#[allow(unused_must_use)]
pub fn print_log(draw_batch: &mut DrawBatch, log_rect: Rect) {
    let mut block =
        TextBlock::new(log_rect.x1 + 1, log_rect.y1 + 1, log_rect.width() - 1, log_rect.height() - 2);

    LOG.lock().iter().rev().take(5).for_each(|log| {
        let mut buf = TextBuilder::empty();

        buf.fg(WHITE).append("> ");
        log.iter().for_each(|frag| {
            buf.fg(frag.color).bg(BLACK).line_wrap(&frag.text);
        });

        buf.ln();
        block.print(&buf);
        // buf.reset();
    });

    block.render_to_draw_batch(draw_batch);
}

pub fn clone_log() -> Vec<Vec<LogFragment>> { LOG.lock().clone() }

pub fn restore_log(log: &mut Vec<Vec<LogFragment>>) {
    LOG.lock().clear();
    LOG.lock().append(log);
}
