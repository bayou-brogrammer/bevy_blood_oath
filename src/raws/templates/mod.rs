use super::*;
use core::fmt::Debug;
use std::any::Any;

mod item_templates;
mod mob_templates;
mod prop_templates;
mod spawn_table_templates;

pub use item_templates::*;
pub use mob_templates::*;
pub use prop_templates::*;
pub use spawn_table_templates::*;

pub trait BaseRawComponent: Debug + Clone {
    fn name(&self) -> String;
    fn glyph(&self) -> Option<&RawGlyph>;
    fn as_any(&self) -> &dyn Any;
}

#[macro_export]
macro_rules! impl_raw {
    ($to:ty) => {
        impl BaseRawComponent for $to {
            fn name(&self) -> String { self.name.clone() }
            fn glyph(&self) -> Option<&RawGlyph> { self.glyph.as_ref() }
            fn as_any(&self) -> &dyn std::any::Any { self }
        }
    };
}
