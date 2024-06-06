pub mod cell;
pub mod cells;
pub mod field_screen;
pub mod gui;
pub mod start_screen;

use gui::GameGui;
use iced::{Application, Settings};
fn main() -> iced::Result {
    GameGui::run(Settings::default())
}
