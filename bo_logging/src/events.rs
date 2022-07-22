use lazy_static::lazy_static;
use parking_lot::Mutex;
use std::collections::HashMap;

lazy_static! {
    static ref EVENTS: Mutex<HashMap<String, i32>> = Mutex::new(HashMap::new());
}

pub fn clear_events() {
    EVENTS.lock().clear();
}

pub fn record_event<T: ToString>(event: T, n: i32) {
    let event_name = event.to_string();
    let mut events = EVENTS.lock();
    if let Some(e) = events.get_mut(&event_name) {
        *e += n;
    } else {
        events.insert(event_name, n);
    }
}

pub fn get_event_count<T: ToString>(event: T) -> i32 {
    let event_name = event.to_string();
    let events = EVENTS.lock();
    if let Some(e) = events.get(&event_name) {
        *e
    } else {
        0
    }
}

pub fn clone_events() -> HashMap<String, i32> {
    EVENTS.lock().clone()
}

pub fn load_events(events: HashMap<String, i32>) {
    EVENTS.lock().clear();
    events.iter().for_each(|(k, v)| {
        EVENTS.lock().insert(k.to_string(), *v);
    });
}
