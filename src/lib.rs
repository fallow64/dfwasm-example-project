#![no_main]
#![no_std]

use alloc::vec::Vec;

#[macro_use]
extern crate alloc;

mod df;
mod fmt;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Full,
}

struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Grid {
    fn new(width: usize, height: usize, cells: Vec<Cell>) -> Self {
        Grid {
            width,
            height,
            cells,
        }
    }

    fn get_cell(&self, x: usize, y: usize) -> Option<Cell> {
        self.cells.get(y * self.width + x).copied()
    }

    fn get_neighor_count(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = (x as isize + dx) as usize;
                let ny = (y as isize + dy) as usize;

                if self.get_cell(nx, ny) == Some(Cell::Full) {
                    count += 1;
                }
            }
        }

        count
    }

    fn tick(&mut self) {
        let mut new_cells = self.cells.clone();

        for x in 0..self.width {
            for y in 0..self.height {
                let neighbor_count = self.get_neighor_count(x, y);

                let cell = self.get_cell(x, y).unwrap_or(Cell::Empty);
                let new_cell = match (cell, neighbor_count) {
                    (Cell::Empty, 3) => Cell::Full,
                    (Cell::Full, 2) | (Cell::Full, 3) => Cell::Full,
                    _ => Cell::Empty,
                };

                new_cells[y * self.width + x] = new_cell;
            }
        }

        self.cells = new_cells;
    }

    pub fn print(&self) {
        for row in self.cells.chunks(self.width) {
            for cell in row {
                match cell {
                    Cell::Empty => print!("  "),
                    Cell::Full => print!("#"),
                }
            }
            df::putc('\n');
        }
    }

    pub fn set_full(&mut self, x: usize, y: usize) {
        if let Some(cell) = self.cells.get_mut(y * self.width + x) {
            *cell = Cell::Full;
        }
    }

    pub fn set_empty(&mut self, x: usize, y: usize) {
        if let Some(cell) = self.cells.get_mut(y * self.width + x) {
            *cell = Cell::Empty;
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    let width = 20;
    let height = 20;
    let cells = vec![Cell::Empty; width * height];

    let mut grid = Grid::new(width, height, cells);
    // glider
    grid.set_full(2, 0);
    grid.set_full(0, 1);
    grid.set_full(2, 1);
    grid.set_full(1, 2);
    grid.set_full(2, 2);

    loop {
        let start_time = df::millis();
        grid.print();
        let print_time = df::millis();
        grid.tick();
        let end_time = df::millis();

        let print_duration = print_time - start_time;
        let tick_duration = end_time - print_time;
        println!("Print time: ", print_duration, "ms");
        println!("Update time: ", tick_duration, "ms");
        df::wait(1000);
    }
}

#[panic_handler]
#[cfg(target_arch = "wasm32")]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    core::arch::wasm32::unreachable();
}
