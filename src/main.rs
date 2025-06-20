//! Game of Life demonstration using the new modular structure

use game_of_life::prelude::*;
use game_of_life::grid::StandardGrid;
use std::io::{self, Write};
use std::{thread, time};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Game of Life Optimization Demo");
    println!("==============================");
    
    // Create the original pattern from the old implementation
    let initial_state = [
        "⬜███⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜",
        "⬜██⬜⬜██⬜⬜██⬜⬜███⬜██",
        "██⬜█⬜⬜█⬜⬜⬜⬜█⬜⬜██⬜⬜⬜",
        "⬜⬜⬜█⬜⬜██⬜█⬜⬜█⬜⬜██⬜⬜",
        "⬜██⬜⬜█⬜█⬜⬜⬜██⬜█⬜⬜█⬜",
        "⬜⬜███⬜⬜⬜█⬜███⬜██⬜██",
        "⬜⬜⬜⬜⬜⬜██⬜⬜█⬜███⬜██⬜",
    ];
    
    // Convert the pattern to our new format
    let grid = StandardGrid::from_string_pattern(&initial_state, '█', '⬜')?;
    let mut engine = auto_from_grid_ultimate_engine(&grid as &dyn Grid);

    println!("\nRunning visual simulation with Ultimate Engine...");
    println!("Grid size: {}x{}", engine.width(), engine.height());
    println!("Initial live cells: {}", engine.count_live_cells());
    
    // Run visual simulation
    print!("\x1b[?1049h"); // Enter alternate screen
    io::stdout().flush().unwrap();
    
    let frame_duration = time::Duration::from_millis(400);
    
    for step in 0..9 {
        print!("\x1b[H"); // Move cursor to top
        print!("\x1b[2J"); // Clear screen
        
        println!("Step: {} | Live cells: {}", step, engine.count_live_cells());
        print_grid_from_engine(&engine);
        
        io::stdout().flush().unwrap();
        thread::sleep(frame_duration);
        
        engine.step();
    }
    
    thread::sleep(time::Duration::from_millis(2000));
    print!("\x1b[?1049l"); // Exit alternate screen
    io::stdout().flush().unwrap();
    
    println!("\nSimulation complete!");
    println!("Ultimate Engine features demonstrated:");
    println!("- Bit-packed representation (64 cells per u64)");
    println!("- SIMD parallelism for massive speedup");
    println!("- Advanced bit manipulation algorithms");
    println!("- Multi-threading with Rayon");
    println!("\nFor benchmarks, run: cargo run --example ultimate_comparison --release");
    
    Ok(())
}

fn print_grid_from_engine(engine: &Box<dyn GameOfLifeEngine>) {
    let grid = engine.get_grid();
    let mut output = String::new();
    for row in 0..grid.height() {
        for col in 0..grid.width() {
            let cell = grid.get_cell(row, col);
            let square = if cell { "⬛" } else { "⬜" };
            output.push_str(square);
        }
        output.push('\n');
    }
    print!("{}", output);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_pattern_conversion() {
        let pattern = ["⬜█⬜", "█⬜█", "⬜█⬜"];
        let grid = StandardGrid::from_string_pattern(&pattern, '█', '⬜').unwrap();
        
        assert_eq!(grid.width(), 3);
        assert_eq!(grid.height(), 3);
        assert_eq!(grid.count_live_cells(), 4);
        
        // Check specific cells
        assert!(!grid.get_cell(0, 0)); // ⬜
        assert!(grid.get_cell(0, 1));  // █
        assert!(!grid.get_cell(0, 2)); // ⬜
    }
    
    #[test]
    fn test_ultimate_engine_functionality() {
        let pattern = ["...", "###", "..."];
        let grid = StandardGrid::from_string_pattern(&pattern, '#', '.').unwrap();
        let mut engine = UltimateEngine::<4>::from_grid(&grid);
        
        // Initial state: horizontal line
        assert_eq!(engine.count_live_cells(), 3);
        
        // After one step: should become vertical line
        engine.step();
        assert_eq!(engine.count_live_cells(), 3);
        
        // After another step: back to horizontal
        engine.step();
        assert_eq!(engine.count_live_cells(), 3);
    }
    
    #[test]
    fn test_engine_equivalence() {
        // Test that Ultimate and Naive engines produce the same results
        let pattern = [".....", ".###.", ".....", ".###.", "....."];
        let grid = StandardGrid::from_string_pattern(&pattern, '#', '.').unwrap();
        
        let mut naive_engine = NaiveEngine::from_grid(&grid as &dyn Grid);
        let mut ultimate_engine = UltimateEngine::<4>::from_grid(&grid as &dyn Grid);
        
        // Run for several steps and verify they stay in sync
        for step in 0..5 {
            assert_eq!(
                naive_engine.get_grid().count_live_cells(),
                ultimate_engine.count_live_cells(),
                "Engines diverged at step {}", step
            );
            
            naive_engine.step();
            ultimate_engine.step();
        }
    }
}
