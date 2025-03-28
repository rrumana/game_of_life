use rayon::prelude::*;

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
            // Convert the character to a digit. If conversion fails, assume the cell is dead.
            let cell = ch.to_digit(10).unwrap_or(0) as u8;
            grid.push(cell);
        }
    }
    grid
}

fn main() {
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

    // Parse the initial state into a 1D grid.
    let mut grid = parse_initial_state(&initial_state);
    let mut next_grid = vec![0u8; WIDTH * HEIGHT];

    // Initialize the entire 20x20 grid using 1s and 0s


    for _ in 0..10 { // simulate 10 generations
        update(&grid, &mut next_grid);
        std::mem::swap(&mut grid, &mut next_grid);
    }

    // Print the final state of the grid
    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            let cell = grid[index(row, col)];
            let square = if cell == 1 { "⬛" } else { "⬜" };
            print!("{}", square);        }
        println!();
    }
}
