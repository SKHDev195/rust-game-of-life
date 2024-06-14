use std::mem;

use iced::{
    widget::{column, row, Button, Row, Text},
    window, Command, Element, Theme,
};

#[derive(Debug, Clone)]
pub enum StopWarningWindowMessage {
    StopGame,
    GoBackToGame,
}

#[derive(Debug)]
pub struct StopWarningWindow {
    title: String,
    pub(crate) window_id: window::Id,
}

impl StopWarningWindow {
    pub fn new() -> Self {
        Self {
            title: String::from("Warning!"),
            window_id: window::Id::unique(),
        }
    }

    pub fn view(&self) -> Element<StopWarningWindowMessage> {
        let confirm_button =
            Button::new(Text::new("Yes")).on_press(StopWarningWindowMessage::StopGame);
        let deny_button =
            Button::new(Text::new("No")).on_press(StopWarningWindowMessage::GoBackToGame);
        let expl_text = Text::new("Stop the game and go back to the setup screen?");
        let buttons_row = row![confirm_button, deny_button]
            .spacing(20)
            .align_items(iced::Alignment::Center)
            .padding(10)
            .height(70);
        let res = column![expl_text, buttons_row].spacing(20).into();
        res
    }

    pub fn update(&mut self, message: StopWarningWindowMessage) {
        match message {
            _ => mem::drop(self),
        }
    }
}
