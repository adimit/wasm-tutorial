use bitvec::prelude::*;
use std::fmt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Universe {
    edge_size: usize,
    cells: BitVec<Msb0, u16>,
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (pos, cell) in self.cells.iter().enumerate() {
            let c = if *cell { "█" } else { "░" };
            let newline = if pos % self.edge_size == self.edge_size - 1 {
                "\n"
            } else {
                ""
            };
            write!(f, "{}{}", c, newline)?;
        }
        Ok(())
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn width(&self) -> usize {
        self.edge_size
    }

    pub fn height(&self) -> usize {
        self.edge_size
    }

    pub fn cells(&self) -> *const u16 {
        self.cells.as_ptr()
    }

    pub fn flip(&mut self, x: usize, y: usize) {
        let index = self.index(x, y);
        self.flip_index(index);
    }

    pub fn tick(&mut self) {
        let mut flip_indices: Vec<(usize, usize)> = Vec::new();
        for x in 0..self.edge_size {
            for y in 0..self.edge_size {
                if self.index_has_to_be_flipped(x, y) {
                    flip_indices.push((x, y));
                }
            }
        }
        for (x, y) in flip_indices {
            self.flip(x, y);
        }
    }
}

impl Universe {
    pub fn build_universe(edge_size: usize) -> Universe {
        let cells = bitvec![Msb0, u16; 0; edge_size * edge_size];
        Universe { edge_size, cells }
    }

    fn index(&self, x: usize, y: usize) -> usize {
        (self.edge_size * (y % self.edge_size)) + (x % self.edge_size)
    }

    fn flip_index(&mut self, index: usize) {
        let old = self.cells[index];
        self.cells.set(index, !old);
    }

    fn get(&self, x: usize, y: usize) -> bool {
        self.cells[self.index(x, y)]
    }

    fn get_neighbour_indices(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let row_above = ((self.edge_size - 1) + y) % self.edge_size;
        let row_below = (y + 1) % self.edge_size;
        let column_to_the_left = ((self.edge_size - 1) + x) % self.edge_size;
        let column_to_the_right = (x + 1) % self.edge_size;
        vec![
            (column_to_the_left, row_above),
            (x, row_above),
            (column_to_the_right, row_above),
            (column_to_the_left, y),
            (column_to_the_right, y),
            (column_to_the_left, row_below),
            (x, row_below),
            (column_to_the_right, row_below),
        ]
    }

    fn get_amount_of_live_neighbours(&self, x: usize, y: usize) -> u8 {
        self.get_neighbour_indices(x, y)
            .iter()
            .cloned()
            .map(|(x, y)| self.get(x, y) as u8)
            .sum()
    }

    fn index_has_to_be_flipped(&self, x: usize, y: usize) -> bool {
        let live_neighbours = self.get_amount_of_live_neighbours(x, y);
        match (self.get(x, y), live_neighbours) {
            (true, 2) => false,
            (true, 3) => false,
            (true, _) => true,
            (false, 3) => true,
            (false, _) => false,
        }
    }
}

#[test]
fn get_amount_of_live_neighbours_works() {
    let mut universe = Universe::build_universe(4);
    assert_eq!(universe.get_amount_of_live_neighbours(0, 0), 0);
    universe.flip(0, 1);
    assert_eq!(universe.get_amount_of_live_neighbours(0, 0), 1);
    universe.flip(0, 3);
    assert_eq!(universe.get_amount_of_live_neighbours(0, 0), 2);
    universe.flip(1, 3);
    assert_eq!(universe.get_amount_of_live_neighbours(0, 0), 3);
    universe.flip(3, 3);
    assert_eq!(universe.get_amount_of_live_neighbours(0, 0), 4);
}

#[test]
fn get_neighbour_indices_wraps_correctly() {
    let universe = Universe::build_universe(5);
    assert_eq!(
        universe.get_neighbour_indices(0, 0),
        vec![
            (4, 4),
            (0, 4),
            (1, 4),
            (4, 0),
            (1, 0),
            (4, 1),
            (0, 1),
            (1, 1)
        ]
    );
    assert_eq!(
        universe.get_neighbour_indices(4, 4),
        vec![
            (3, 3),
            (4, 3),
            (0, 3),
            (3, 4),
            (0, 4),
            (3, 0),
            (4, 0),
            (0, 0)
        ]
    );
}

#[test]
fn build_universe_has_correct_size() {
    let universe = Universe::build_universe(5);
    assert_eq!(universe.cells.len(), 25);
}

#[test]
fn get_with_large_index_wraps() {
    let mut universe = Universe::build_universe(4);
    universe.flip(4, 4);
    assert_eq!(universe.get(0, 0), true);
}

#[test]
fn universe_can_flip_cell() {
    let mut universe = Universe::build_universe(2);
    universe.flip(0, 1);
    assert_eq!(universe.get(0, 1), true);
}

#[test]
fn lone_cell_dies_in_one_tick() {
    let mut universe = Universe::build_universe(3);
    universe.flip(1, 1);
    universe.tick();
    assert_eq!(universe.get(1, 1), false);
}

#[test]
fn blocks_are_stable() {
    let mut universe = Universe::build_universe(4);
    universe.flip(0, 0);
    universe.flip(0, 1);
    universe.flip(1, 1);
    universe.flip(1, 0);
    universe.tick();
    universe.tick();
    universe.tick();
    universe.tick();
    universe.tick();

    assert_eq!(universe.get(0, 0), true);
    assert_eq!(universe.get(1, 1), true);
    assert_eq!(universe.get(1, 0), true);
    assert_eq!(universe.get(0, 1), true);
    assert_eq!(
        universe
            .cells
            .iter()
            .filter(|cell| { **cell == true })
            .count(),
        4
    );
}

#[test]
fn blinkers_are_stable() {
    let mut universe = Universe::build_universe(5);

    universe.flip(1, 1);
    universe.flip(1, 2);
    universe.flip(1, 3);

    universe.tick();

    assert_eq!(universe.get(0, 2), true);
    assert_eq!(universe.get(1, 2), true);
    assert_eq!(universe.get(2, 2), true);

    universe.tick();

    assert_eq!(universe.get(1, 1), true);
    assert_eq!(universe.get(1, 2), true);
    assert_eq!(universe.get(1, 3), true);
}

#[test]
fn gliders_wrap() {
    let mut universe = Universe::build_universe(5);

    universe.flip(0, 2);
    universe.flip(1, 2);
    universe.flip(2, 2);
    universe.flip(2, 1);
    universe.flip(1, 0);

    (0..20).for_each(|x| {
        universe.tick();
        println!("{}{}", universe.to_string(), x);
    });

    assert_eq!(universe.get(0, 2), true);
    assert_eq!(universe.get(1, 2), true);
    assert_eq!(universe.get(2, 2), true);
    assert_eq!(universe.get(2, 1), true);
    assert_eq!(universe.get(1, 0), true);
}
