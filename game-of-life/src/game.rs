use crate::{cell::Cell, cells::get_cell_neighbours, cells::CellsNeighbours};
use core::num;
use indexmap::{self, IndexMap};
use rand::Rng;

enum GameStatus {
    Stopped,
    Paused,
    Going,
}

fn fill_cells_neighbours(cells_number: &i32, x: &i32, y: &i32) -> CellsNeighbours {
    let mut cn_map: IndexMap<Cell, [i32; 8]> = IndexMap::new();
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
