use grid::*;

pub const LOCATIONS: [(i32, i32); 8] = [
    (0, -1),  // top
    (1, -1),  // top-right
    (1, 0),   // right
    (1, 1),   // bottom-right
    (0, 1),   // bottom
    (-1, 1),  // bottom-left
    (-1, 0),  // left
    (-1, -1), // top-left
];

pub fn is_cell_alive(cell_grid: &Grid<i32>, col: i32, row: i32) -> bool {
    let x = col as usize;
    let y = row as usize;

    let cell_state = cell_grid[y][x];

    cell_state == 1
}

pub fn count_neighbours(cell_grid: &Grid<i32>, col: i32, row: i32) -> i32 {
    let mut neighbours = 0;

    for point in LOCATIONS.iter() {
        let x = point.0 + col;
        let y = point.1 + row;

        if is_cell_alive(cell_grid, x, y) {
            neighbours += 1;
        }

    }

    neighbours
}
