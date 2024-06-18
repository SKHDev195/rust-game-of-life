pub mod cell;
pub mod cells;
pub mod field_screen;
pub mod flow_controls;
pub mod gui;
pub mod start_screen;
pub mod stop_warning_window;

use gui::GameGui;
use iced::multi_window::{self, Application};
use iced::Settings;
fn main() -> iced::Result {
    GameGui::run(Settings::default())
}
