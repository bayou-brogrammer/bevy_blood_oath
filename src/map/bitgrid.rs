use crate::prelude::*;
use bitvec::{
    prelude::*,
    slice::{Iter, IterMut},
};

/// A width-by-height&-sized BitVec for convenient handling of a grid of boolean values.
#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct BitGrid {
    width: i32,
    height: i32,
    bv: BitVec,
}

impl BitGrid {
    /// Create a new BitGrid with the given width and height.
    pub fn new(width: i32, height: i32) -> Self {
        assert!(width >= 0);
        assert!(height >= 0);

        Self { width, height, bv: bitvec![0; (width * height) as usize] }
    }

    pub fn iter(&self) -> Iter<'_, usize, Lsb0> {
        self.bv.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, usize, Lsb0> {
        self.bv.iter_mut()
    }

    pub fn as_bitslice(&mut self) -> &BitSlice {
        self.bv.as_bitslice()
    }

    pub fn as_mut_bitslice(&mut self) -> &mut BitSlice {
        self.bv.as_mut_bitslice()
    }

    /// Reset all elements to false.
    pub fn zero_out_bits(&mut self) {
        self.bv.set_elements(0);
    }

    pub fn apply_all_bits(&mut self) {
        self.bv.as_mut_bitslice().iter_mut().for_each(|mut b| b.set(true));
    }

    /// Get the bool at the given x and y.
    ///
    /// Returns false if out of bounds.
    #[inline]
    pub fn get_bit(&self, pt: Point) -> bool {
        if !self.in_bounds(pt) {
            false
        } else {
            self.bv[self.point2d_to_index(pt)]
        }
    }

    /// Set the bool at the given x and y to value.
    ///
    /// Panics if out of bounds.
    #[inline]
    pub fn set_bit(&mut self, pt: Point, value: bool) {
        let idx = self.point2d_to_index(pt);
        self.bv.set(idx, value);
    }
}

impl Algorithm2D for BitGrid {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }

    fn in_bounds(&self, pos: Point) -> bool {
        pos.x >= 0 && pos.x < self.width as i32 && pos.y > 0 && pos.y < self.height as i32
    }
}

impl BaseMap for BitGrid {}
