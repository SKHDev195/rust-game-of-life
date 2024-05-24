use crate::field::{self, Field};
use crate::{cell::Cell, cells::get_cell_neighbours, cells::Cells};
use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;

extern crate rand;
use rand::Rng;

fn fill_cells_neighbours(x: &i32, y: &i32) -> Cells {
    let mut cn_map: HashMap<i32, [i32; 8]> = HashMap::new();
    let num_of_cells: i32 = x * y;
    let mut cells: Vec<Cell> = Vec::new();
    for i in 0..num_of_cells {
        let alive_chance: i32 = rand::thread_rng().gen_range(0..100);
        match alive_chance {
            ac if ac > 70 => {
                cells.push(Cell::Alive);
                cn_map.insert(i, get_cell_neighbours(&i, x, y));
            }
            _ => {
                cells.push(Cell::Dead);
                cn_map.insert(i, get_cell_neighbours(&i, x, y));
            }
        }
    }

    let res = Cells {
        cells_neighbours_map: cn_map,
        game_cells: cells,
    };

    res
}

pub fn game_of_life(x: i32, y: i32) {
    let mut cells = fill_cells_neighbours(&x, &y);
    let mut field = Field { x, y, cells };
    loop {
        field::render_field(&cells.game_cells, &y);
        field.update();
        sleep(Duration::from_millis(100));
    }
}
