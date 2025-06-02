use rayon::prelude::*;
use rayon::slice::ParallelSliceMut;
use std::io::{self, Write};
use std::{thread, time};

const WIDTH: usize = 20;
const HEIGHT: usize = 20;

fn index(row: usize, col: usize) -> usize {
    row * WIDTH + col
}

fn count_neighbors(grid: &[u8], row: usize, col: usize) -> u8 {
    let mut count = 0;
    for dr in [-1, 0, 1].iter() {
        for dc in [-1, 0, 1].iter() {
            if *dr == 0 && *dc == 0 {
                continue;
            }
            let r = row as isize + dr;
            let c = col as isize + dc;
            if r >= 0 && r < HEIGHT as isize && c >= 0 && c < WIDTH as isize {
                count += grid[index(r as usize, c as usize)];
            }
        }
    }
    count
}

fn update(current: &[u8], next: &mut [u8]) {
    next.par_chunks_mut(WIDTH)
        .into_par_iter()
        .enumerate()
        .for_each(|(row, row_slice)| {
            for col in 0..WIDTH {
                let idx = index(row, col);
                let neighbors = count_neighbors(current, row, col);
                row_slice[col] = match (current[idx], neighbors) {
                    (1, 2) | (1, 3) | (0, 3) => 1,
                    _ => 0,
                };
            }
        });
}

fn parse_initial_state(initial: &[&str]) -> Vec<u8> {
    let mut grid = Vec::with_capacity(WIDTH * HEIGHT);
    for row in initial {
        for ch in row.chars() {
            let cell = ch.to_digit(10).unwrap_or(0) as u8;
            grid.push(cell);
        }
    }
    grid
}

fn print_grid(grid: &[u8]) {
    let mut output = String::new();
    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            let cell = grid[index(row, col)];
            let square = if cell == 1 { "⬛" } else { "⬜" };
            output.push_str(square);
        }
        output.push('\n');
    }
    print!("{}", output);
}

fn main() {

    print!("\x1b[?1049h");
    io::stdout().flush().unwrap();

    let initial_state = [
        "00100000000000000000",
        "10100000000000000111",
        "01100000000000000000",
        "00000000001100000000",
        "00000000001100000010",
        "00100000000000000010",
        "10100000000000000010",
        "01100000000000000000",
        "00000000000000000000",
        "00000000000000000111",
        "00100000000000000000",
        "10100000000000000000",
        "01100000000000000000",
        "00000000000000000000",
        "00000000000000000000",
        "00000000000000000000",
        "00000000000000000000",
        "00000000000000000010",
        "00000000000000000010",
        "00000000000000000010",
    ];

    let mut grid = parse_initial_state(&initial_state);
    let mut next_grid = vec![0u8; WIDTH * HEIGHT];

    for _ in 0..10 { // simulate 10 generations
        update(&grid, &mut next_grid);
        std::mem::swap(&mut grid, &mut next_grid);
    }

    let frame_duration = time::Duration::from_millis(200);

    // Main simulation loop.
    for _ in 0..57 {
        update(&grid, &mut next_grid);
        std::mem::swap(&mut grid, &mut next_grid);
        print!("\x1b[H");
        print_grid(&grid);
        io::stdout().flush().unwrap();
        thread::sleep(frame_duration);
    }

    print!("\x1b[?1049l");
    io::stdout().flush().unwrap();
}
