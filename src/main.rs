use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

use rand::Rng;

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

struct ConwaysGameOfLife {
    world: Vec<Vec<u8>>,
    world_size: usize,
}

impl ConwaysGameOfLife {
    fn new(grid_size: usize) -> ConwaysGameOfLife {
        let mut grid: Vec<Vec<u8>> = vec![vec![0; grid_size + 1]; grid_size + 1];

        let mut rng = rand::thread_rng();

        // Fill the grid with random patterns, skipping the first row and column (padding)
        for i in 1..grid_size {
            for j in 1..grid_size {
                grid[i][j] = rng.gen_range(0..=1); // Randomly assign 0 or 1
            }
        }
        ConwaysGameOfLife {
            world: grid,
            world_size: grid_size,
        }
    }

    fn play(&mut self) {
        while true {
            clear_screen();
            // print current state
            for row in 1..self.world_size {
                for col in 1..self.world_size {
                    if self.world[row][col] == 1 {
                        print!("O ");
                    } else {
                        print!(" ");
                    }
                }
                println!();
            }

            let mut next_state = self.world.clone();
            // compute next state
            for row in 1..self.world_size {
                for col in 1..self.world_size {
                    next_state[row][col] = self.get_new_state(row, col);
                }
            }

            self.world = next_state;
            thread::sleep(Duration::from_millis(50)); // Sleep for a while to see the update
        }
    }

    fn get_new_state(&self, i: usize, j: usize) -> u8 {
        let mut neighbor_live_count = 0;
        neighbor_live_count += self.world[i][j - 1]; // top
        neighbor_live_count += self.world[i][j + 1]; // down
        neighbor_live_count += self.world[i + 1][j]; // right
        neighbor_live_count += self.world[i - 1][j]; // left
        neighbor_live_count += self.world[i + 1][j + 1]; // diagnol 1
        neighbor_live_count += self.world[i - 1][j + 1]; // diagnol 2
        neighbor_live_count += self.world[i + 1][j - 1]; // diagnol 3
        neighbor_live_count += self.world[i - 1][j - 1]; // diagnol 4

        let curr_state = self.world[i][j];

        // Any live cell with fewer than two live neighbours dies

        if curr_state == 1 && neighbor_live_count < 2 {
            return 0;
        }
        // Any live cell with two or three live neighbours lives on to the next generation.
        else if curr_state == 1 && neighbor_live_count == 2 || neighbor_live_count == 3 {
            return 1;
        }
        // Any live cell with more than three live neighbours dies, as if by overpopulation.
        else if curr_state == 1 && neighbor_live_count > 3 {
            return 0;
        }
        // Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
        else if curr_state == 0 && neighbor_live_count == 3 {
            return 1;
        } else {
            return 0;
        }
    }
}

fn main() {
    const GRID_SIZE: usize = 50;
    let mut gol = ConwaysGameOfLife::new(GRID_SIZE);
    gol.play();
}
