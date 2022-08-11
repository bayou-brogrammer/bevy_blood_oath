use crate::prelude::*;

mod builders;
mod common;
mod maps;
mod random;

pub use builders::*;
pub use common::*;
pub use maps::*;
pub use random::*;

////////////////////////////////////////////////////////////////////////////////
// Map Builder Traits
////////////////////////////////////////////////////////////////////////////////

pub trait InitialMapBuilder {
    fn build_map(&mut self, build_data: &mut BuilderMap);
}

pub trait MetaMapBuilder {
    fn build_map(&mut self, build_data: &mut BuilderMap);
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct BuilderMap {
    pub map: Map,
    pub width: i32,
    pub height: i32,
    pub history: Vec<Map>,
    pub rooms: Option<Vec<Rect>>,
    pub spawn_list: Vec<(usize, String)>,
    pub starting_position: Option<Point>,
    pub corridors: Option<Vec<Vec<usize>>>,
}

impl BuilderMap {
    fn take_snapshot(&mut self) {
        if SHOW_MAPGEN_VISUALIZER {
            let mut snapshot = self.map.clone();
            snapshot.revealed.apply_all_bits(true);
            self.history.push(snapshot);
        }
    }
}

pub struct BuilderChain {
    pub build_data: BuilderMap,
    builders: Vec<Box<dyn MetaMapBuilder>>,
    starter: Option<Box<dyn InitialMapBuilder>>,
}

impl BuilderChain {
    pub fn new<S: ToString>(new_depth: i32, width: i32, height: i32, name: S) -> BuilderChain {
        BuilderChain {
            starter: None,
            builders: Vec::new(),
            build_data: BuilderMap {
                width,
                height,
                rooms: None,
                corridors: None,
                history: Vec::new(),
                spawn_list: Vec::new(),
                starting_position: None,
                map: Map::new(new_depth, width, height, name),
            },
        }
    }

    pub fn start_with(&mut self, starter: Box<dyn InitialMapBuilder>) {
        match self.starter {
            None => self.starter = Some(starter),
            Some(_) => panic!("You can only have one starting builder."),
        };
    }

    pub fn with(&mut self, metabuilder: Box<dyn MetaMapBuilder>) { self.builders.push(metabuilder); }

    pub fn build_map(&mut self) {
        match &mut self.starter {
            None => panic!("Cannot run a map builder chain without a starting build system"),
            Some(starter) => {
                // Build the starting map
                starter.build_map(&mut self.build_data);
            }
        }

        // Build additional layers in turn
        for metabuilder in self.builders.iter_mut() {
            metabuilder.build_map(&mut self.build_data);
        }
    }
}

#[allow(clippy::match_single_binding)]
pub fn level_builder(new_depth: i32, width: i32, height: i32) -> BuilderChain {
    console::log(format!("Depth: {}", new_depth));
    match new_depth {
        1 => town_builder(new_depth, width, height),
        _ => random_builder(new_depth, width, height),
    }
}
