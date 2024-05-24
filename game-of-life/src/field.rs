use crate::{
    cell::{self, Cell},
    cells::Cells,
    messages::FieldMessages,
};
use iced::widget::{row, Row};

pub struct Field {
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) cells: Cells,
}

impl Field {
    fn update(&mut self, message: FieldMessages) {
        update_field(&mut self.cells);
    }

    fn view(&self) -> Element<FieldMessages> {
        let mut rows = Vec::new();
        for i in 0..self.y {
            rows.push();
        }
        column![rows]
    }
}

pub fn render_field(cells: &Vec<Cell>, y: &i32) {
    for (i, cell) in cells.iter().enumerate() {
        cell::show_cell_basic(cell);

        if (i as i32 + 1) % *y == 0 {
            print!("\n");
        }
    }
    println!();
}

fn update_field(cells_neighbours: &mut Cells) {
    for i in 0..cells_neighbours.game_cells.len() {
        let index = i as i32;
        let alive_neighbours = get_alive_neighbours(cells_neighbours, &index);
        match alive_neighbours {
            an if an < 2 => {
                cells_neighbours.game_cells[i] = Cell::Dead;
            }
            an if an > 3 => {
                cells_neighbours.game_cells[i] = Cell::Dead;
            }
            an if (an == 2 || an == 3) && cells_neighbours.game_cells[i] == Cell::Alive => {}
            an if an == 3 && cells_neighbours.game_cells[i] == Cell::Dead => {
                cells_neighbours.game_cells[i] = Cell::Alive;
            }
            _ => {}
        }
    }
}

fn get_alive_neighbours(cells_neighbours: &Cells, index: &i32) -> i32 {
    let mut res: i32 = 0;

    if let Some(neighbours) = cells_neighbours.cells_neighbours_map.get(index) {
        for i in 0..8 {
            if neighbours[i] != -1 {
                let neighbour_cell_index = neighbours[i] as usize;
                let neighbour_cell = &cells_neighbours.game_cells[neighbour_cell_index];
                match neighbour_cell {
                    Cell::Alive => res += 1,
                    _ => {}
                }
            }
        }
    }

    res
}
