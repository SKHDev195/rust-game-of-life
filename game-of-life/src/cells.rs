use crate::cell::Cell;
use indexmap;

pub struct CellsNeighbours {
    pub cells_neighbours_map: indexmap::IndexMap<Cell, [i32; 8]>,
}

pub fn get_cell_neighbours(i: &i32, x: &i32, y: &i32) -> [i32; 8] {
    let row = i / y;
    let col = i % y;
    let mut neighbours: [i32; 8] = [-1; 8];

    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for (index, (dr, dc)) in directions.iter().enumerate() {
        let new_row = row as i32 + dr;
        let new_col = col as i32 + dc;

        if new_row >= 0 && new_row < *x && new_col >= 0 && new_col < *y {
            neighbours[index] = new_row * y + new_col;
        }
    }

    neighbours
}
