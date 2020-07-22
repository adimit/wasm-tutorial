#[derive(Clone)]
pub enum CellState {
    ALIVE,
    DEAD
}

pub struct Universe {
    edge_size: usize,
    cells: Vec<CellState>
}

fn tick(universe: &mut Vec<CellState>) {

}

pub fn build_universe(edge_size: usize) -> Universe {
    let cells: Vec<CellState> = vec![CellState::DEAD; edge_size * edge_size];
    Universe { edge_size, cells }
}

#[test]
fn build_universe_has_correct_size() {
    let universe = build_universe(5);
    assert_eq!(universe.cells.len(), 25);
}
