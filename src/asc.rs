//! Ascension scaling tables.
//!
//! This file is a checked-in static snapshot consumed by the simulation core.
//! Regeneration and source validation are intentionally external to this crate.

use crate::ops::EOp;

/// Ascension adjustment for one numeric field of one enemy move.
#[derive(Clone, Copy, Debug)]
pub struct AscOp {
    pub def: u16,
    pub mv: u8,
    pub op: u8,
    pub hits: bool,
    pub gate: u8,
    pub high: i32,
}

/// Enemy move adjustments by ascension tier.
pub static ASC_OPS: &[AscOp] = &[
    AscOp { def: 4, mv: 0, op: 0, hits: false, gate: 9, high: 13 },
    AscOp { def: 4, mv: 1, op: 0, hits: false, gate: 9, high: 7 },
    AscOp { def: 4, mv: 1, op: 1, hits: false, gate: 8, high: 6 },
    AscOp { def: 4, mv: 2, op: 0, hits: false, gate: 9, high: 3 },
    AscOp { def: 5, mv: 2, op: 0, hits: false, gate: 9, high: 15 },
    AscOp { def: 6, mv: 1, op: 0, hits: false, gate: 9, high: 8 },
    AscOp { def: 6, mv: 2, op: 0, hits: false, gate: 9, high: 14 },
    AscOp { def: 7, mv: 1, op: 0, hits: false, gate: 9, high: 8 },
    AscOp { def: 7, mv: 2, op: 0, hits: false, gate: 9, high: 8 },
    AscOp { def: 7, mv: 3, op: 0, hits: false, gate: 9, high: 6 },
    AscOp { def: 9, mv: 0, op: 0, hits: false, gate: 9, high: 4 },
    AscOp { def: 10, mv: 1, op: 0, hits: false, gate: 9, high: 9 },
    AscOp { def: 11, mv: 0, op: 0, hits: false, gate: 9, high: 5 },
    AscOp { def: 12, mv: 1, op: 0, hits: false, gate: 9, high: 12 },
    AscOp { def: 13, mv: 1, op: 0, hits: false, gate: 9, high: 16 },
    AscOp { def: 14, mv: 0, op: 0, hits: false, gate: 9, high: 6 },
    AscOp { def: 14, mv: 0, op: 1, hits: false, gate: 9, high: 6 },
    AscOp { def: 14, mv: 1, op: 0, hits: false, gate: 9, high: 6 },
    AscOp { def: 14, mv: 1, op: 1, hits: false, gate: 9, high: 6 },
    AscOp { def: 14, mv: 2, op: 0, hits: false, gate: 9, high: 13 },
    AscOp { def: 15, mv: 1, op: 0, hits: false, gate: 9, high: 1 },
    AscOp { def: 15, mv: 1, op: 0, hits: true, gate: 9, high: 9 },
    AscOp { def: 16, mv: 0, op: 0, hits: false, gate: 9, high: 9 },
    AscOp { def: 16, mv: 1, op: 0, hits: false, gate: 9, high: 12 },
    AscOp { def: 17, mv: 1, op: 0, hits: false, gate: 9, high: 9 },
    AscOp { def: 17, mv: 2, op: 0, hits: false, gate: 9, high: 16 },
    AscOp { def: 17, mv: 3, op: 0, hits: false, gate: 9, high: 9 },
    AscOp { def: 19, mv: 2, op: 0, hits: false, gate: 9, high: 3 },
    AscOp { def: 19, mv: 3, op: 0, hits: false, gate: 9, high: 3 },
    AscOp { def: 20, mv: 0, op: 0, hits: false, gate: 9, high: 5 },
    AscOp { def: 20, mv: 1, op: 0, hits: false, gate: 9, high: 2 },
    AscOp { def: 20, mv: 2, op: 0, hits: false, gate: 9, high: 3 },
    AscOp { def: 21, mv: 0, op: 0, hits: false, gate: 9, high: 16 },
    AscOp { def: 22, mv: 0, op: 0, hits: false, gate: 9, high: 8 },
    AscOp { def: 22, mv: 0, op: 1, hits: false, gate: 9, high: 8 },
    AscOp { def: 23, mv: 0, op: 0, hits: false, gate: 9, high: 19 },
    AscOp { def: 23, mv: 2, op: 0, hits: false, gate: 9, high: 23 },
    AscOp { def: 23, mv: 3, op: 0, hits: false, gate: 9, high: 16 },
    AscOp { def: 24, mv: 1, op: 0, hits: false, gate: 9, high: 16 },
    AscOp { def: 26, mv: 0, op: 0, hits: false, gate: 9, high: 9 },
    AscOp { def: 27, mv: 0, op: 0, hits: false, gate: 9, high: 10 },
    AscOp { def: 27, mv: 1, op: 0, hits: false, gate: 8, high: 18 },
    AscOp { def: 27, mv: 2, op: 0, hits: false, gate: 9, high: 16 },
    AscOp { def: 28, mv: 1, op: 0, hits: false, gate: 9, high: 15 },
    AscOp { def: 28, mv: 2, op: 0, hits: false, gate: 9, high: 6 },
    AscOp { def: 28, mv: 2, op: 1, hits: false, gate: 9, high: 3 },
    AscOp { def: 29, mv: 1, op: 0, hits: false, gate: 9, high: 17 },
    AscOp { def: 29, mv: 2, op: 0, hits: false, gate: 9, high: 8 },
    AscOp { def: 29, mv: 3, op: 0, hits: false, gate: 9, high: 4 },
    AscOp { def: 30, mv: 1, op: 0, hits: false, gate: 9, high: 25 },
    AscOp { def: 30, mv: 2, op: 0, hits: false, gate: 9, high: 19 },
    AscOp { def: 31, mv: 1, op: 1, hits: false, gate: 9, high: 1 },
    AscOp { def: 31, mv: 2, op: 0, hits: false, gate: 9, high: 4 },
    AscOp { def: 31, mv: 3, op: 0, hits: false, gate: 9, high: 18 },
    AscOp { def: 33, mv: 1, op: 0, hits: false, gate: 9, high: 18 },
    AscOp { def: 33, mv: 2, op: 0, hits: false, gate: 9, high: 9 },
    AscOp { def: 33, mv: 3, op: 0, hits: false, gate: 9, high: 13 },
    AscOp { def: 33, mv: 3, op: 2, hits: false, gate: 9, high: 3 },
    AscOp { def: 34, mv: 0, op: 0, hits: false, gate: 9, high: 14 },
    AscOp { def: 34, mv: 1, op: 0, hits: false, gate: 9, high: 4 },
    AscOp { def: 34, mv: 2, op: 0, hits: false, gate: 9, high: 7 },
    AscOp { def: 34, mv: 3, op: 0, hits: false, gate: 9, high: 3 },
    AscOp { def: 34, mv: 4, op: 0, hits: false, gate: 9, high: 14 },
    AscOp { def: 35, mv: 0, op: 0, hits: false, gate: 9, high: 4 },
    AscOp { def: 35, mv: 1, op: 0, hits: false, gate: 9, high: 20 },
    AscOp { def: 35, mv: 2, op: 0, hits: false, gate: 9, high: 3 },
    AscOp { def: 35, mv: 3, op: 0, hits: false, gate: 9, high: 35 },
    AscOp { def: 36, mv: 0, op: 0, hits: false, gate: 9, high: 16 },
    AscOp { def: 36, mv: 2, op: 0, hits: false, gate: 9, high: 5 },
    AscOp { def: 37, mv: 0, op: 0, hits: false, gate: 9, high: 4 },
    AscOp { def: 38, mv: 1, op: 0, hits: false, gate: 9, high: 8 },
    AscOp { def: 38, mv: 2, op: 0, hits: false, gate: 9, high: 13 },
    AscOp { def: 39, mv: 1, op: 0, hits: false, gate: 9, high: 7 },
    AscOp { def: 40, mv: 1, op: 0, hits: false, gate: 9, high: 5 },
    AscOp { def: 41, mv: 0, op: 0, hits: false, gate: 9, high: 20 },
    AscOp { def: 41, mv: 1, op: 0, hits: false, gate: 9, high: 160 },
    AscOp { def: 41, mv: 4, op: 0, hits: false, gate: 9, high: 17 },
    AscOp { def: 41, mv: 5, op: 0, hits: false, gate: 9, high: 19 },
    AscOp { def: 41, mv: 5, op: 1, hits: false, gate: 9, high: 4 },
    AscOp { def: 42, mv: 0, op: 0, hits: true, gate: 9, high: 4 },
    AscOp { def: 42, mv: 1, op: 0, hits: false, gate: 9, high: 9 },
    AscOp { def: 43, mv: 0, op: 0, hits: true, gate: 9, high: 8 },
    AscOp { def: 43, mv: 0, op: 0, hits: false, gate: 9, high: 3 },
    AscOp { def: 43, mv: 1, op: 0, hits: false, gate: 9, high: 20 },
    AscOp { def: 44, mv: 0, op: 0, hits: false, gate: 9, high: 17 },
    AscOp { def: 44, mv: 1, op: 0, hits: false, gate: 9, high: 13 },
    AscOp { def: 44, mv: 1, op: 1, hits: false, gate: 9, high: 13 },
    AscOp { def: 44, mv: 2, op: 0, hits: false, gate: 9, high: 6 },
    AscOp { def: 44, mv: 3, op: 0, hits: false, gate: 9, high: 10 },
    AscOp { def: 44, mv: 3, op: 1, hits: false, gate: 9, high: 3 },
    AscOp { def: 44, mv: 3, op: 2, hits: false, gate: 8, high: 22 },
    AscOp { def: 45, mv: 1, op: 0, hits: false, gate: 9, high: 5 },
    AscOp { def: 47, mv: 1, op: 0, hits: false, gate: 9, high: 9 },
    AscOp { def: 47, mv: 2, op: 0, hits: false, gate: 9, high: 31 },
    AscOp { def: 47, mv: 3, op: 0, hits: false, gate: 9, high: 3 },
    AscOp { def: 47, mv: 4, op: 0, hits: false, gate: 9, high: 9 },
    AscOp { def: 48, mv: 1, op: 0, hits: false, gate: 9, high: 15 },
    AscOp { def: 49, mv: 0, op: 0, hits: false, gate: 9, high: 32 },
    AscOp { def: 49, mv: 1, op: 0, hits: false, gate: 9, high: 12 },
    AscOp { def: 49, mv: 2, op: 0, hits: false, gate: 9, high: 2 },
    AscOp { def: 50, mv: 1, op: 0, hits: false, gate: 9, high: 18 },
    AscOp { def: 51, mv: 0, op: 0, hits: false, gate: 9, high: 4 },
    AscOp { def: 51, mv: 1, op: 0, hits: false, gate: 9, high: 4 },
    AscOp { def: 52, mv: 0, op: 0, hits: false, gate: 9, high: 17 },
    AscOp { def: 52, mv: 1, op: 0, hits: false, gate: 9, high: 4 },
    AscOp { def: 52, mv: 3, op: 0, hits: false, gate: 9, high: 36 },
    AscOp { def: 53, mv: 1, op: 0, hits: false, gate: 9, high: 21 },
    AscOp { def: 53, mv: 2, op: 0, hits: false, gate: 9, high: 13 },
    AscOp { def: 55, mv: 0, op: 0, hits: false, gate: 9, high: 15 },
    AscOp { def: 58, mv: 0, op: 0, hits: false, gate: 9, high: 19 },
    AscOp { def: 58, mv: 1, op: 0, hits: false, gate: 9, high: 4 },
    AscOp { def: 58, mv: 1, op: 0, hits: true, gate: 9, high: 3 },
    AscOp { def: 59, mv: 1, op: 0, hits: false, gate: 9, high: 19 },
    AscOp { def: 59, mv: 2, op: 0, hits: false, gate: 9, high: 8 },
    AscOp { def: 61, mv: 0, op: 0, hits: false, gate: 9, high: 14 },
    AscOp { def: 61, mv: 1, op: 0, hits: false, gate: 9, high: 23 },
    AscOp { def: 61, mv: 3, op: 0, hits: false, gate: 9, high: 40 },
    AscOp { def: 62, mv: 0, op: 0, hits: false, gate: 9, high: 15 },
    AscOp { def: 62, mv: 1, op: 0, hits: false, gate: 9, high: 14 },
    AscOp { def: 62, mv: 2, op: 0, hits: false, gate: 9, high: 10 },
    AscOp { def: 63, mv: 0, op: 0, hits: false, gate: 9, high: 31 },
    AscOp { def: 63, mv: 1, op: 0, hits: false, gate: 9, high: 7 },
    AscOp { def: 63, mv: 1, op: 0, hits: true, gate: 9, high: 4 },
    AscOp { def: 63, mv: 2, op: 0, hits: false, gate: 9, high: 19 },
    AscOp { def: 64, mv: 1, op: 0, hits: false, gate: 9, high: 22 },
    AscOp { def: 64, mv: 2, op: 0, hits: false, gate: 9, high: 16 },
    AscOp { def: 64, mv: 6, op: 0, hits: false, gate: 9, high: 5 },
    AscOp { def: 64, mv: 6, op: 1, hits: false, gate: 9, high: 3 },
    AscOp { def: 64, mv: 6, op: 1, hits: false, gate: 9, high: 3 },
    AscOp { def: 65, mv: 0, op: 0, hits: false, gate: 9, high: 18 },
    AscOp { def: 65, mv: 1, op: 0, hits: false, gate: 9, high: 4 },
    AscOp { def: 66, mv: 1, op: 1, hits: false, gate: 9, high: 6 },
    AscOp { def: 67, mv: 0, op: 0, hits: false, gate: 9, high: 9 },
    AscOp { def: 68, mv: 0, op: 0, hits: false, gate: 9, high: 5 },
    AscOp { def: 68, mv: 1, op: 0, hits: false, gate: 9, high: 7 },
    AscOp { def: 68, mv: 2, op: 0, hits: true, gate: 9, high: 3 },
    AscOp { def: 69, mv: 0, op: 0, hits: false, gate: 9, high: 20 },
    AscOp { def: 69, mv: 1, op: 0, hits: false, gate: 9, high: 16 },
    AscOp { def: 69, mv: 2, op: 0, hits: false, gate: 9, high: 11 },
    AscOp { def: 69, mv: 3, op: 0, hits: false, gate: 8, high: 15 },
    AscOp { def: 69, mv: 4, op: 0, hits: false, gate: 9, high: 23 },
    AscOp { def: 69, mv: 5, op: 0, hits: false, gate: 9, high: 14 },
    AscOp { def: 70, mv: 0, op: 0, hits: false, gate: 9, high: 15 },
    AscOp { def: 70, mv: 1, op: 0, hits: false, gate: 8, high: 37 },
    AscOp { def: 70, mv: 2, op: 0, hits: false, gate: 9, high: 26 },
    AscOp { def: 71, mv: 1, op: 0, hits: false, gate: 9, high: 11 },
    AscOp { def: 71, mv: 3, op: 0, hits: false, gate: 9, high: 7 },
    AscOp { def: 71, mv: 3, op: 1, hits: false, gate: 9, high: 7 },
    AscOp { def: 72, mv: 1, op: 0, hits: false, gate: 9, high: 17 },
    AscOp { def: 73, mv: 0, op: 0, hits: false, gate: 9, high: 9 },
    AscOp { def: 73, mv: 1, op: 0, hits: false, gate: 9, high: 12 },
    AscOp { def: 73, mv: 2, op: 0, hits: false, gate: 9, high: 7 },
    AscOp { def: 74, mv: 1, op: 0, hits: false, gate: 9, high: 9 },
    AscOp { def: 75, mv: 0, op: 0, hits: false, gate: 9, high: 13 },
    AscOp { def: 75, mv: 2, op: 0, hits: false, gate: 8, high: 8 },
    AscOp { def: 75, mv: 2, op: 1, hits: false, gate: 9, high: 2 },
    AscOp { def: 76, mv: 1, op: 0, hits: false, gate: 9, high: 11 },
    AscOp { def: 77, mv: 0, op: 0, hits: false, gate: 9, high: 6 },
    AscOp { def: 77, mv: 1, op: 0, hits: false, gate: 9, high: 3 },
    AscOp { def: 78, mv: 0, op: 0, hits: false, gate: 9, high: 11 },
    AscOp { def: 79, mv: 0, op: 0, hits: false, gate: 9, high: 9 },
    AscOp { def: 79, mv: 1, op: 0, hits: false, gate: 9, high: 7 },
    AscOp { def: 80, mv: 1, op: 0, hits: false, gate: 9, high: 6 },
    AscOp { def: 80, mv: 2, op: 0, hits: false, gate: 9, high: 16 },
    AscOp { def: 81, mv: 1, op: 0, hits: false, gate: 9, high: 17 },
    AscOp { def: 81, mv: 2, op: 0, hits: false, gate: 9, high: 8 },
    AscOp { def: 81, mv: 4, op: 0, hits: false, gate: 9, high: 15 },
    AscOp { def: 82, mv: 1, op: 0, hits: false, gate: 9, high: 5 },
    AscOp { def: 82, mv: 3, op: 0, hits: false, gate: 9, high: 33 },
    AscOp { def: 83, mv: 0, op: 0, hits: false, gate: 9, high: 30 },
    AscOp { def: 83, mv: 3, op: 0, hits: false, gate: 9, high: 40 },
    AscOp { def: 84, mv: 0, op: 0, hits: false, gate: 9, high: 14 },
    AscOp { def: 84, mv: 1, op: 0, hits: false, gate: 9, high: 7 },
    AscOp { def: 84, mv: 2, op: 0, hits: false, gate: 9, high: 17 },
    AscOp { def: 85, mv: 0, op: 0, hits: false, gate: 9, high: 16 },
    AscOp { def: 85, mv: 1, op: 0, hits: false, gate: 9, high: 6 },
    AscOp { def: 86, mv: 0, op: 1, hits: false, gate: 9, high: 2 },
    AscOp { def: 86, mv: 1, op: 0, hits: false, gate: 9, high: 5 },
    AscOp { def: 87, mv: 0, op: 2, hits: false, gate: 9, high: 2 },
    AscOp { def: 87, mv: 1, op: 0, hits: false, gate: 9, high: 15 },
    AscOp { def: 88, mv: 0, op: 0, hits: false, gate: 9, high: 17 },
    AscOp { def: 88, mv: 1, op: 0, hits: false, gate: 9, high: 10 },
    AscOp { def: 89, mv: 1, op: 0, hits: false, gate: 9, high: 17 },
    AscOp { def: 89, mv: 2, op: 0, hits: false, gate: 9, high: 4 },
    AscOp { def: 90, mv: 0, op: 0, hits: false, gate: 9, high: 7 },
    AscOp { def: 90, mv: 0, op: 1, hits: false, gate: 8, high: 9 },
    AscOp { def: 90, mv: 2, op: 0, hits: false, gate: 9, high: 11 },
    AscOp { def: 90, mv: 3, op: 0, hits: false, gate: 8, high: 9 },
    AscOp { def: 90, mv: 4, op: 0, hits: false, gate: 9, high: 40 },
    AscOp { def: 91, mv: 1, op: 0, hits: false, gate: 9, high: 18 },
    AscOp { def: 92, mv: 1, op: 0, hits: false, gate: 9, high: 21 },
    AscOp { def: 92, mv: 2, op: 0, hits: false, gate: 9, high: 10 },
    AscOp { def: 92, mv: 3, op: 0, hits: false, gate: 9, high: 14 },
    AscOp { def: 92, mv: 3, op: 1, hits: false, gate: 8, high: 14 },
    AscOp { def: 93, mv: 0, op: 0, hits: false, gate: 9, high: 8 },
    AscOp { def: 93, mv: 1, op: 0, hits: false, gate: 9, high: 7 },
    AscOp { def: 93, mv: 2, op: 0, hits: false, gate: 9, high: 30 },
    AscOp { def: 94, mv: 0, op: 0, hits: false, gate: 9, high: 4 },
    AscOp { def: 94, mv: 1, op: 0, hits: false, gate: 9, high: 3 },
    AscOp { def: 94, mv: 2, op: 0, hits: false, gate: 9, high: 11 },
    AscOp { def: 95, mv: 0, op: 0, hits: false, gate: 9, high: 16 },
    AscOp { def: 95, mv: 1, op: 0, hits: false, gate: 9, high: 16 },
    AscOp { def: 95, mv: 2, op: 0, hits: false, gate: 9, high: 11 },
    AscOp { def: 95, mv: 2, op: 1, hits: false, gate: 9, high: 4 },
    AscOp { def: 95, mv: 3, op: 0, hits: false, gate: 9, high: 8 },
    AscOp { def: 96, mv: 1, op: 0, hits: false, gate: 9, high: 14 },
    AscOp { def: 96, mv: 2, op: 0, hits: false, gate: 9, high: 5 },
    AscOp { def: 97, mv: 0, op: 1, hits: false, gate: 9, high: 4 },
    AscOp { def: 97, mv: 1, op: 0, hits: false, gate: 9, high: 8 },
    AscOp { def: 98, mv: 0, op: 0, hits: false, gate: 9, high: 11 },
    AscOp { def: 98, mv: 1, op: 0, hits: false, gate: 9, high: 14 },
    AscOp { def: 98, mv: 2, op: 0, hits: false, gate: 9, high: 4 },
    AscOp { def: 99, mv: 0, op: 0, hits: false, gate: 8, high: 8 },
    AscOp { def: 99, mv: 1, op: 0, hits: false, gate: 8, high: 7 },
    AscOp { def: 99, mv: 2, op: 0, hits: false, gate: 8, high: 9 },
    AscOp { def: 100, mv: 1, op: 0, hits: false, gate: 9, high: 10 },
    AscOp { def: 102, mv: 0, op: 0, hits: false, gate: 9, high: 7 },
    AscOp { def: 102, mv: 1, op: 0, hits: false, gate: 9, high: 9 },
    AscOp { def: 102, mv: 2, op: 0, hits: false, gate: 9, high: 18 },
    AscOp { def: 103, mv: 0, op: 0, hits: false, gate: 9, high: 11 },
    AscOp { def: 104, mv: 0, op: 0, hits: false, gate: 9, high: 8 },
];

/// Apply the high-ascension adjustment for one enemy move operation.
/// Below A8 the operation is returned unchanged.
#[inline]
pub fn adjust(def: u16, mv: usize, op_ix: usize, asc: u8, op: EOp) -> EOp {
    if asc < 8 {
        return op;
    }
    for r in ASC_OPS {
        if r.def == def && r.mv as usize == mv && r.op as usize == op_ix && asc >= r.gate {
            return patch(op, r.high, r.hits);
        }
    }
    op
}

/// 把新值写回 `EOp` 的对应字段。**认不出的 op 原样返回** ——
/// 生成器只会为它认识的那几种 op 建行，这里是兜底。
fn patch(op: EOp, v: i32, hits_field: bool) -> EOp {
    match op {
        EOp::Attack { base, hits } => {
            if hits_field {
                EOp::Attack { base, hits: v }
            } else {
                EOp::Attack { base: v, hits }
            }
        }
        EOp::AttackPlusStackHits { base, hits, per } => {
            if hits_field {
                EOp::AttackPlusStackHits { base, hits: v, per }
            } else {
                EOp::AttackPlusStackHits { base: v, hits, per }
            }
        }
        EOp::AttackPlusSelfStatus { base, hits, per } => {
            if hits_field {
                EOp::AttackPlusSelfStatus { base, hits: v, per }
            } else {
                EOp::AttackPlusSelfStatus { base: v, hits, per }
            }
        }
        EOp::Block(_) => EOp::Block(v),
        EOp::SelfStatus { st, .. } => EOp::SelfStatus { st, amt: v },
        EOp::PlayerStatus { st, .. } => EOp::PlayerStatus { st, amt: v },
        EOp::AddCardToDiscard { card, .. } => EOp::AddCardToDiscard { card, count: v },
        EOp::Heal(_) => EOp::Heal(v),
        other => other,
    }
}

