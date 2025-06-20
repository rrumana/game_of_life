#![feature(portable_simd)]
#![feature(array_windows)]
#![feature(array_chunks)]

pub mod engines;
pub mod grid;
pub mod benchmark;

pub use engines::{GameOfLifeEngine, EngineInfo};
pub use grid::Grid;

pub mod prelude {
    pub use crate::engines::{GameOfLifeEngine, EngineInfo};
    pub use crate::grid::{Grid, StandardGrid};
    pub use crate::engines::naive::NaiveEngine;
    pub use crate::engines::ultimate::{UltimateEngine, auto_new_ultimate_engine, auto_from_grid_ultimate_engine, safe_auto_new_ultimate_engine, create_optimal_engine};
}