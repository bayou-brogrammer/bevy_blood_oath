use super::*;
use bracket_terminal::prelude::*;

#[derive(Default)]
pub struct Logger {
    current_color: RGB,
    fragments: Vec<LogFragment>,
}

impl Logger {
    pub fn new() -> Self {
        Logger { current_color: RGB::named(WHITE), fragments: Vec::new() }
    }

    pub fn log(self) {
        append_entry(self.fragments)
    }

    pub fn color(mut self, color: (u8, u8, u8)) -> Self {
        self.current_color = RGB::named(color);
        self
    }

    pub fn append<T: ToString>(mut self, text: T) -> Self {
        self.fragments.push(LogFragment { color: self.current_color, text: text.to_string() });
        self
    }

    pub fn append_with_color<T: ToString>(mut self, text: T, color: (u8, u8, u8)) -> Self {
        self.fragments.push(LogFragment { text: text.to_string(), color: RGB::named(color) });
        self
    }

    pub fn npc_name<T: ToString>(mut self, text: T) -> Self {
        self.fragments.push(LogFragment { text: text.to_string(), color: RGB::named(YELLOW) });
        self
    }

    pub fn item_name<T: ToString>(mut self, text: T) -> Self {
        self.fragments.push(LogFragment { color: RGB::named(CYAN), text: text.to_string() });
        self
    }

    pub fn damage(mut self, damage: i32) -> Self {
        self.fragments.push(LogFragment { color: RGB::named(RED), text: format!("{}", damage) });
        self
    }

    pub fn healing(mut self, heal_amount: i32) -> Self {
        self.fragments.push(LogFragment { color: RGB::named(GREEN), text: format!("{}", heal_amount) });
        self
    }
}
