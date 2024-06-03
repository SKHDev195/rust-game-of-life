mod cell;
mod cells;
mod field;
mod messages;

use field::Field;
use iced::{Application, Settings};
fn main() -> iced::Result {
    Field::run(Settings::default())
}
