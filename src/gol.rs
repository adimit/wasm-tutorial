use std::fmt;

#[repr(u8)]
#[derive(Clone, Debug, PartialEq, Copy, Eq)]
pub enum CellState {
    DEAD = 0,
    ALIVE = 1
}

impl CellState {
    fn flip(&self) -> CellState {
        match self {
            CellState::ALIVE => CellState::DEAD,
            CellState::DEAD => CellState::ALIVE
        }
    }
}

pub struct Universe {
    edge_size: usize,
    cells: Vec<CellState>
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (pos, cell) in self.cells.iter().enumerate() {
            let c = if *cell == CellState::ALIVE { "█" } else { "░" };
            let newline = if pos % self.edge_size == self.edge_size - 1 { "\n" } else { "" };
            write!(f, "{}{}", c, newline)?;
        }
        Ok(())
    }
}

impl Universe {
    fn index(&self, x: usize, y: usize) -> usize {
        (self.edge_size * (y % self.edge_size)) + (x % self.edge_size)
    }

    fn flip(&mut self, x: usize, y: usize) {
        let index = self.index(x, y);
        self.flip_index(index);
    }

    fn flip_index(&mut self, index: usize) {
        self.cells[index] = self.cells[index].flip();
    }

    fn get(&self, x: usize, y: usize) -> CellState {
        self.cells[self.index(x, y)]
    }

    fn get_neighbour_indices(&self, x: usize, y: usize) -> Vec<(usize,usize)> {
        let row_above = ((self.edge_size - 1) + y )% self.edge_size;
        let row_below = (y + 1) % self.edge_size;
        let column_to_the_left  = ((self.edge_size - 1) + x ) % self.edge_size;
        let column_to_the_right = (x + 1) % self.edge_size;
        vec![
            (column_to_the_left, row_above),
            (x, row_above),
            (column_to_the_right, row_above),
            (column_to_the_left, y),
            (column_to_the_right, y),
            (column_to_the_left, row_below),
            (x, row_below),
            (column_to_the_right, row_below)
        ]
    }

    fn get_amount_of_live_neighbours(&self, x: usize, y: usize) -> u8 {
        self.get_neighbour_indices(x,y).iter().cloned().map(|(x, y)| { self.get(x, y) as u8 }).sum()
    }

    fn index_has_to_be_flipped(&self, x: usize, y: usize) -> bool {
        let live_neighbours = self.get_amount_of_live_neighbours(x, y);
        match (self.get(x,y), live_neighbours) {
            (CellState::ALIVE, 2) => false,
            (CellState::ALIVE, 3) => false,
            (CellState::ALIVE, _) => true,
            (CellState::DEAD, 3) => true,
            (CellState::DEAD, _) => false
        }
    }

    fn tick(&mut self) {
        let mut flip_indices: Vec<(usize, usize)> = Vec::new();
        for x in 0..self.edge_size-1 {
            for y in 0..self.edge_size-1 {
                if self.index_has_to_be_flipped(x,y) {
                    flip_indices.push((x,y));
                }
            }
        }
        for (x,y) in flip_indices {
            self.flip(x,y);
        }
    }
}

pub fn build_universe(edge_size: usize) -> Universe {
    let cells: Vec<CellState> = vec![CellState::DEAD; edge_size * edge_size];
    Universe { edge_size, cells }
}

#[test]
fn get_amount_of_live_neighbours_works() {
    let mut universe = build_universe(4);
    assert_eq!(universe.get_amount_of_live_neighbours(0,0), 0);
    universe.flip(0, 1);
    assert_eq!(universe.get_amount_of_live_neighbours(0,0), 1);
    universe.flip(0, 3);
    assert_eq!(universe.get_amount_of_live_neighbours(0,0), 2);
    universe.flip(1, 3);
    assert_eq!(universe.get_amount_of_live_neighbours(0,0), 3);
    universe.flip(3, 3);
    assert_eq!(universe.get_amount_of_live_neighbours(0,0), 4);
}

#[test]
fn get_neighbour_indices_wraps_correctly() {
    let universe = build_universe(5);
    assert_eq!(universe.get_neighbour_indices(0,0),
               vec![(4, 4), (0, 4), (1, 4), (4, 0), (1, 0), (4, 1), (0, 1), (1, 1)]
    );
    assert_eq!(universe.get_neighbour_indices(4,4),
               vec![(3, 3), (4, 3), (0, 3), (3, 4), (0, 4), (3, 0), (4, 0), (0, 0)]
    );
}

#[test]
fn build_universe_has_correct_size() {
    let universe = build_universe(5);
    assert_eq!(universe.cells.len(), 25);
}

#[test]
fn get_with_large_index_wraps() {
    let mut universe = build_universe(4);
    universe.flip(4, 4);
    assert_eq!(universe.get(0, 0), CellState::ALIVE);
}

#[test]
fn universe_can_flip_cell() {
    let mut universe = build_universe(2);
    universe.flip(0, 1);
    assert_eq!(universe.get(0, 1), CellState::ALIVE);
}

#[test]
fn lone_cell_dies_in_one_tick() {
    let mut universe = build_universe(3);
    universe.flip(1,1);
    universe.tick();
    assert_eq!(universe.get(1,1), CellState::DEAD);
}

#[test]
fn blocks_are_stable() {
    let mut universe = build_universe(4);
    universe.flip(0,0);
    universe.flip(0,1);
    universe.flip(1,1);
    universe.flip(1,0);
    universe.tick();
    universe.tick();
    universe.tick();
    universe.tick();
    universe.tick();

    assert_eq!(universe.get(0,0), CellState::ALIVE);
    assert_eq!(universe.get(1,1), CellState::ALIVE);
    assert_eq!(universe.get(1,0), CellState::ALIVE);
    assert_eq!(universe.get(0,1), CellState::ALIVE);
    assert_eq!(universe.cells.iter().filter(|cell| { **cell == CellState::ALIVE }).count(), 4);
}

#[test]
fn blinkers_are_stable() {
    let mut universe = build_universe(5);

    universe.flip(1,1);
    universe.flip(1,2);
    universe.flip(1,3);

    universe.tick();

    assert_eq!(universe.get(0,2), CellState::ALIVE);
    assert_eq!(universe.get(1,2), CellState::ALIVE);
    assert_eq!(universe.get(2,2), CellState::ALIVE);

    universe.tick();

    assert_eq!(universe.get(1,1), CellState::ALIVE);
    assert_eq!(universe.get(1,2), CellState::ALIVE);
    assert_eq!(universe.get(1,3), CellState::ALIVE);
}
