use iced::widget::{checkbox, column, pick_list, row, text_input, Button, Column, Text, TextInput};
use iced::{Command, Element, Renderer};

#[derive(Debug, Clone)]
pub enum StartingScreenMessage {
    SetColums(String),
    SetRows(String),
    StartGame,
}

pub struct StartingScreenController {
    pub x: i32,
    pub y: i32,
}

impl StartingScreenController {
    pub fn new() -> Self {
        Self { x: 10, y: 10 }
    }

    pub fn update(&mut self, message: StartingScreenMessage) -> Command<StartingScreenMessage> {
        match message {
            StartingScreenMessage::SetColums(x) => {
                self.x = x.parse::<i32>().unwrap();
                Command::none()
            }
            StartingScreenMessage::SetRows(y) => {
                self.y = y.parse::<i32>().unwrap();
                Command::none()
            }
            _ => Command::none(),
        }
    }

    pub fn view(&self) -> Element<StartingScreenMessage> {
        let column_input = TextInput::new("Number of columns", &self.x.to_string())
            .on_input(StartingScreenMessage::SetColums);
        let row_input = TextInput::new("Number of rows", &self.y.to_string())
            .on_input(StartingScreenMessage::SetRows);
        let start_game_button =
            Button::new(Text::new("Start Game")).on_press(StartingScreenMessage::StartGame);
        column![column_input, row_input, start_game_button]
            .spacing(15)
            .align_items(iced::Alignment::Center)
            .padding(10)
            .into()
    }
}
