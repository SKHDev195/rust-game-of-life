use crate::cell::Cell;
use std::collections::HashMap;

pub struct CellsNeighbours {
    pub cells_neighbours_map: HashMap<Cell, [i32; 8]>,
}
