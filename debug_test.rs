//! Debug test to understand the correctness issue

use game_of_life::prelude::*;

fn main() {
    println!("Debug Test: Ultimate Engine Correctness");
    println!("=======================================");
    
    // Create a simple 3x3 grid with a blinker pattern
    let pattern = [
        "...",
        "###",
        "...",
    ];
    
    let grid = StandardGrid::from_string_pattern(&pattern, '#', '.').unwrap();
    
    // Test with Naive Engine
    let mut naive_engine = NaiveEngine::from_grid(&grid);
    println!("Naive Engine - Initial state:");
    print_grid_state(&naive_engine);
    
    naive_engine.step();
    println!("Naive Engine - After 1 step:");
    print_grid_state(&naive_engine);
    
    // Test with Ultimate Engine
    let mut ultimate_engine = UltimateEngine::from_grid(&grid);
    println!("\nUltimate Engine - Initial state:");
    print_ultimate_state(&ultimate_engine);
    
    ultimate_engine.step();
    println!("Ultimate Engine - After 1 step:");
    print_ultimate_state(&ultimate_engine);
}

fn print_grid_state(engine: &NaiveEngine) {
    println!("Live cells: {}", engine.count_live_cells());
    for row in 0..engine.height() {
        for col in 0..engine.width() {
            print!("{}", if engine.get_cell(row, col) { '#' } else { '.' });
        }
        println!();
    }
}

fn print_ultimate_state(engine: &UltimateEngine) {
    println!("Live cells: {}", engine.count_live_cells());
    for row in 0..engine.height() {
        for col in 0..engine.width() {
            print!("{}", if engine.get_cell(row, col) { '#' } else { '.' });
        }
        println!();
    }
}