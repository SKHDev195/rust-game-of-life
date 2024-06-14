use iced::{
    widget::{row, Button, Text},
    Command, Element, Length,
};

use crate::field_screen::FieldScreenMessage;

pub struct FlowControlsController {
    pause_button
}

impl FlowControlsController {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, _message: FieldScreenMessage) -> Command<FieldScreenMessage> {
        Command::none()
    }

    pub fn view(&self) -> Element<FieldScreenMessage> {
        let pause_button = Button::new(Text::new("Pause")).on_press(FieldScreenMessage::Pause);
        let play_button = Button::new(Text::new("Play")).on_press(FieldScreenMessage::Play);
        let stop_button =
            Button::new(Text::new("Stop")).on_press(FieldScreenMessage::DisplayWarningWindow);
        row![pause_button, play_button, stop_button]
            .spacing(20)
            .align_items(iced::Alignment::Center)
            .padding(10)
            .height(80)
            .into()
    }
}
