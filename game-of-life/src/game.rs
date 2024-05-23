use crate::{cell::Cell, cells::CellsNeighbours};
use core::num;
use rand::Rng;
use std::collections::HashMap;

enum GameStatus {
    Stopped,
    Paused,
    Going,
}

fn fill_cells_neighbours(cells_number: i32, x: i32, y: i32) -> CellsNeighbours {
    let mut cn_map: HashMap<Cell, [i32; 8]> = HashMap::new();
    let num_of_cells: i32 = x * y;

    for i in 0..num_of_cells {
        let alive_chance: i32 = rand::thread_rng().gen_range(0..100);
        match alive_chance {
            ac if ac > 70 => {
                cn_map.insert(Cell::Alive, get_cell_neighbours(i, x, y));
            }
            _ => {
                cn_map.insert(Cell::Dead, get_cell_neighbours(i, x, y));
            }
        }
    }

    let res = CellsNeighbours {
        cells_neighbours_map: cn_map,
    };

    res
}

pub fn get_cell_neighbours(i: i32, x: i32, y: i32) -> [i32; 8] {
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

        if new_row >= 0 && new_row < x && new_col >= 0 && new_col < y {
            neighbours[index] = new_row * y + new_col;
        }
    }

    neighbours
}
