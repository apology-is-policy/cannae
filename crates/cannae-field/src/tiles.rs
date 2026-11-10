//! A sparse grid: square tiles of cells, allocated only where there are men.
//!
//! A battle fills a small part of the ground it ranges over, and a rout scatters men for
//! kilometres, so no array may span the bounding box. Cells are addressed as
//! `tile << CELL_BITS | row << TILE_SHIFT | column`; within a tile the neighbour to the east is
//! `+1` and the neighbour to the north is `+TILE`. Callers keep their own arrays, indexed by
//! address and `cells()` long.
//!
//! Tiles are numbered in the order they are first touched. Nothing a caller computes may depend
//! on that order, only on which cells exist, so the numbering cannot change a result.

pub const TILE_SHIFT: u32 = 5;
/// Cells along one side of a tile.
pub const TILE: i32 = 1 << TILE_SHIFT;
pub const CELL_BITS: u32 = 2 * TILE_SHIFT;
pub const TILE_CELLS: usize = 1 << CELL_BITS;
/// No such cell.
pub const NONE: u32 = u32::MAX;

const MASK: i32 = TILE - 1;
const SMALLEST: usize = 64;

#[derive(Clone, Debug)]
pub struct Tiles {
    /// Open addressing, linear probing. A slot is free when its value is `NONE`.
    keys: Vec<u64>,
    vals: Vec<u32>,
    coords: Vec<(i32, i32)>,
    last: (u64, u32),
}

impl Default for Tiles {
    fn default() -> Self {
        Tiles {
            keys: vec![0; SMALLEST],
            vals: vec![NONE; SMALLEST],
            coords: Vec::new(),
            last: (0, NONE),
        }
    }
}

fn key(tx: i32, ty: i32) -> u64 {
    ((ty as u32 as u64) << 32) | tx as u32 as u64
}

/// A cell's place within its tile.
pub fn local(cx: i32, cy: i32) -> u32 {
    (((cy & MASK) << TILE_SHIFT) | (cx & MASK)) as u32
}

impl Tiles {
    /// Forgets every tile. The table shrinks again after a rout has come and gone.
    pub fn clear(&mut self) {
        if self.vals.len() > SMALLEST && self.coords.len() * 16 < self.vals.len() {
            let cap = self.vals.len() / 2;
            self.keys.truncate(cap);
            self.vals.truncate(cap);
        }
        self.vals.fill(NONE);
        self.coords.clear();
        self.last = (0, NONE);
    }

    pub fn len(&self) -> usize {
        self.coords.len()
    }

    pub fn is_empty(&self) -> bool {
        self.coords.is_empty()
    }

    /// How long a caller's per-cell array must be.
    pub fn cells(&self) -> usize {
        self.coords.len() * TILE_CELLS
    }

    fn slot(&self, k: u64) -> usize {
        (k.wrapping_mul(0x9E37_79B9_7F4A_7C15) >> 40) as usize & (self.vals.len() - 1)
    }

    pub fn find_tile(&self, tx: i32, ty: i32) -> Option<u32> {
        let k = key(tx, ty);
        let mut s = self.slot(k);
        loop {
            let v = self.vals[s];
            if v == NONE {
                return None;
            }
            if self.keys[s] == k {
                return Some(v);
            }
            s = (s + 1) & (self.vals.len() - 1);
        }
    }

    #[inline]
    pub fn touch_tile(&mut self, tx: i32, ty: i32) -> u32 {
        let k = key(tx, ty);
        if self.last.1 != NONE && self.last.0 == k {
            return self.last.1;
        }
        self.touch_slow(k, tx, ty)
    }

    fn touch_slow(&mut self, k: u64, tx: i32, ty: i32) -> u32 {
        let mut s = self.slot(k);
        let t = loop {
            let v = self.vals[s];
            if v == NONE {
                let t = self.coords.len() as u32;
                self.keys[s] = k;
                self.vals[s] = t;
                self.coords.push((tx, ty));
                if self.coords.len() * 2 > self.vals.len() {
                    self.grow();
                }
                break t;
            }
            if self.keys[s] == k {
                break v;
            }
            s = (s + 1) & (self.vals.len() - 1);
        };
        self.last = (k, t);
        t
    }

    fn grow(&mut self) {
        let cap = self.vals.len() * 2;
        self.keys = vec![0; cap];
        self.vals = vec![NONE; cap];
        for (t, &(tx, ty)) in self.coords.iter().enumerate() {
            let k = key(tx, ty);
            let mut s = self.slot(k);
            while self.vals[s] != NONE {
                s = (s + 1) & (cap - 1);
            }
            self.keys[s] = k;
            self.vals[s] = t as u32;
        }
    }

    /// The address of a cell, allocating its tile if need be.
    pub fn touch(&mut self, cx: i32, cy: i32) -> u32 {
        (self.touch_tile(cx >> TILE_SHIFT, cy >> TILE_SHIFT) << CELL_BITS) | local(cx, cy)
    }

    /// The address of a cell, or `NONE` where no tile has been touched.
    pub fn find(&self, cx: i32, cy: i32) -> u32 {
        match self.find_tile(cx >> TILE_SHIFT, cy >> TILE_SHIFT) {
            Some(t) => (t << CELL_BITS) | local(cx, cy),
            None => NONE,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cells_keep_their_addresses_and_their_neighbours() {
        let mut t = Tiles::default();
        let a = t.touch(3, 4);
        assert_eq!(t.touch(3, 4), a);
        assert_eq!(t.find(3, 4), a);
        assert_eq!(t.find(4, 4), a + 1);
        assert_eq!(t.find(3, 5), a + TILE as u32);
        assert_eq!(t.find(3, TILE), NONE);
        // Negative ground: the cell west of column 0 is the last column of the tile to the west.
        let b = t.touch(-1, 4);
        assert_eq!(b >> CELL_BITS, 1);
        assert_eq!(b & (TILE_CELLS as u32 - 1), local(TILE - 1, 4));
        assert_eq!(t.len(), 2);
        assert_eq!(t.cells(), 2 * TILE_CELLS);
    }

    #[test]
    fn the_table_grows_and_forgets() {
        let mut t = Tiles::default();
        for k in 0..1_000 {
            t.touch(k * TILE, -k * TILE);
        }
        assert_eq!(t.len(), 1_000);
        for k in 0..1_000 {
            assert_eq!(t.find(k * TILE, -k * TILE) >> CELL_BITS, k as u32);
        }
        // The tile whose two coordinates are both -1 has an all-ones key; it is still a tile.
        let odd = t.touch(-1, -1);
        assert_eq!(t.find(-1, -1), odd);
        t.clear();
        assert!(t.is_empty());
        assert_eq!(t.find(0, 0), NONE);
        assert_eq!(t.touch(7 * TILE, 0) >> CELL_BITS, 0);
    }
}
