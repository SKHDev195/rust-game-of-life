use iced::widget::{checkbox, column, pick_list, row, text_input, Button, Text, TextInput};
use iced::{Command, Element, Renderer};

#[derive(Debug, Clone)]
pub enum StartingScreenMessage {
    SetColums(String),
    SetRows(String),
    StartGame,
}

pub struct StartingScreenController {
    x: i32,
    y: i32,
}

impl StartingScreenController {
    pub fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn update(&mut self, message: StartingScreenMessage) {
        match message {
            StartingScreenMessage::SetColums(x) => {
                self.x = x.parse::<i32>().unwrap();
            }
            StartingScreenMessage::SetRows(y) => {
                self.y = y.parse::<i32>().unwrap();
            }
            _ => {}
        }
    }

    pub fn view(&self) -> Element<'_, StartingScreenMessage, Renderer> {
        let column_input = TextInput::new("Number of columns", &self.x.to_string())
            .on_input(StartingScreenMessage::SetColums);
        let row_input = TextInput::new("Number of rows", &self.y.to_string())
            .on_input(StartingScreenMessage::SetRows);
        let start_game_button = Button::new(Text::new("Start Game"));

        let settings_column = 
    }
}
