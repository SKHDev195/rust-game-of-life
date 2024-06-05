pub mod cell;
pub mod cells;
pub mod field;
pub mod gui;
pub mod messages;
pub mod start_screen;

use field::Field;
use iced::{Application, Settings};
fn main() -> iced::Result {
    Field::run(Settings::default())
}
