use crate::cell::{self, Cell};
use crate::cells::CellsNeighbours;

pub struct Field {
    x: i32,
    y: i32,
}

pub fn render_field(cells: , y: &i32) {
    let cells: Vec<_> = cells_neighbours.cells_neighbours_map.keys().collect();
    for i in 0..cells.len() {
        match i {
            i if i != *y as usize => {
                cell::show_cell(cells[i]);
            }
            _ => {
                cell::show_cell(cells[i]);
                print!("\n");
            }
        }
    }
}

pub fn update_field(cells_neighbours: &CellsNeighbours) {
    let cells: Vec<_> = cells_neighbours.cells_neighbours_map.keys().collect();
    for c in cells_neighbours.cells_neighbours_map.keys() {
        let alive_neighbours = get_alive_neighbours(cells_neighbours, &c);
        match alive_neighbours {
            an if an < 2 => 
        }
    }

}

fn get_alive_neighbours(cells_neighbours: &CellsNeighbours, cell: &Cell) -> usize {
    let mut res = 0;

    if let Some(neighbours) = cells_neighbours.cells_neighbours_map.get(cell) {
        for i in 0..8 {
            if neighbours[i] != -1 {
                let neighbour_cell_index = neighbours[i] as usize;
                let neighbour_cell = cells_neighbours
                    .cells_neighbours_map
                    .get_index(neighbour_cell_index);
                if let Some((cell_key, _)) = neighbour_cell {
                    if *cell_key == Cell::Alive {
                        res += 1;
                    }
                }
            }
        }
    }

    res
}
