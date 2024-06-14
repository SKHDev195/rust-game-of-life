use std::collections::HashMap;

use crate::{
    cell::Cell,
    cells::{get_cell_neighbours, Cells},
    flow_controls::{self, FlowControlsController},
    stop_warning_window::{StopWarningWindow, StopWarningWindowMessage},
};
use iced::{
    widget::{Column, Row},
    window, Command, Element, Size,
};
use rand::Rng;

#[derive(Debug, Clone, PartialEq)]
pub enum FieldScreenMessage {
    Update,
    Pause,
    Play,
    DisplayWarningWindow,
}

pub enum ActiveStatus {
    Paused,
    Playing,
}

pub struct FieldScreenController {
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) cells: Cells,
    pub(crate) active_status: ActiveStatus,
    flow_controls: FlowControlsController,
}

impl FieldScreenController {
    pub fn new(x: &i32, y: &i32) -> Self {
        let cells = fill_cells_neighbours(x, y);
        let flow_controls = FlowControlsController::new();
        let res = Self {
            x: *x,
            y: *y,
            cells,
            active_status: ActiveStatus::Paused,
            flow_controls,
        };
        res
    }

    pub fn update(&mut self, message: FieldScreenMessage) -> Command<FieldScreenMessage> {
        match message {
            FieldScreenMessage::Update => {
                update_field(&mut self.cells);
                Command::none()
            }
            FieldScreenMessage::Play => {
                self.active_status = ActiveStatus::Playing;
                Command::none()
            }
            FieldScreenMessage::Pause => {
                self.active_status = ActiveStatus::Paused;
                Command::none()
            }
            FieldScreenMessage::DisplayWarningWindow => {
                self.active_status = ActiveStatus::Paused;
                Command::none()
            }
        }
    }

    pub fn view(&self) -> Element<FieldScreenMessage> {
        let mut controls = Vec::new();
        let mut start_index: usize = 0;
        for _ in 0..self.y {
            let end_index = start_index + self.x as usize;
            let row_cells = &self.cells.game_cells[start_index..end_index];

            let row =
                Row::with_children(row_cells.iter().map(|cell| cell.view()).collect::<Vec<_>>());
            controls.push(row.into());

            start_index = end_index;
        }
        controls.push(self.flow_controls.view());
        Column::with_children(controls)
            .align_items(iced::Alignment::Center)
            .into()
    }
}

fn update_field(cells_neighbours: &mut Cells) {
    let mut new_cells = cells_neighbours.game_cells.clone();

    for i in 0..cells_neighbours.game_cells.len() {
        let index = i as i32;
        let alive_neighbours = get_alive_neighbours(cells_neighbours, &index);
        new_cells[i] = match (cells_neighbours.game_cells[i], alive_neighbours) {
            (Cell::Alive, x) if x < 2 => Cell::Dead,
            (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
            (Cell::Alive, x) if x > 3 => Cell::Dead,
            (Cell::Dead, 3) => Cell::Alive,
            (otherwise, _) => otherwise,
        };
    }

    cells_neighbours.game_cells = new_cells;
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
            ac if ac > 90 => {
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
