use std::collections::HashMap;

use crate::{cell::Cell, cells::get_cell_neighbours, cells::Cells, messages::FieldMessages};
use iced::{
    executor,
    time::{self, Duration},
    widget::{Column, Row},
    Application, Command, Element, Sandbox, Subscription, Theme,
};
use rand::Rng;

pub struct Field {
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) cells: Cells,
}

impl Application for Field {
    type Executor = executor::Default;
    type Message = FieldMessages;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<FieldMessages>) {
        let cells = fill_cells_neighbours(&400, &300);
        let res = Self {
            x: 400,
            y: 300,
            cells: cells,
        };
        (res, Command::none())
    }

    fn title(&self) -> String {
        String::from("Game of Life")
    }

    fn update(&mut self, _message: FieldMessages) -> Command<FieldMessages> {
        update_field(&mut self.cells);
        Command::none()
    }

    fn view(&self) -> Element<FieldMessages> {
        let mut rows = Vec::new();
        let mut start_index: usize = 0;
        for _ in 0..self.y {
            let end_index = start_index + self.x as usize;
            let row_cells = &self.cells.game_cells[start_index..end_index];

            let row =
                Row::with_children(row_cells.iter().map(|cell| cell.view()).collect::<Vec<_>>());
            rows.push(row.into());

            start_index = end_index;
        }
        Column::with_children(rows)
            .align_items(iced::Alignment::Center)
            .into()
    }

    fn subscription(&self) -> Subscription<FieldMessages> {
        iced::time::every(Duration::from_millis(500)).map(|_| FieldMessages::Update)
    }
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

fn fill_cells_neighbours(x: &i32, y: &i32) -> Cells {
    let mut cn_map: HashMap<i32, [i32; 8]> = HashMap::new();
    let num_of_cells: i32 = x * y;
    let mut cells: Vec<Cell> = Vec::new();
    for i in 0..num_of_cells {
        let alive_chance: i32 = rand::thread_rng().gen_range(0..100);
        match alive_chance {
            ac if ac > 80 => {
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
