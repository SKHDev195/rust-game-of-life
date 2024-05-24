#[derive(Eq, Hash, PartialEq)]
pub enum Cell {
    Dead,
    Alive,
}

pub fn show_cell(c: &Cell) {
    match c {
        Cell::Alive => {
            print!("#");
        }
        _ => {
            print!("_")
        }
    }
}
