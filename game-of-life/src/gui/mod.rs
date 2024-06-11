use iced::{executor, time::Duration, Application, Command, Renderer, Theme};

use crate::{
    field_screen::{self, FieldScreenController, FieldScreenMessage},
    start_screen::{StartingScreenController, StartingScreenMessage},
};

enum ActiveScreen {
    StartingScreen,
    FieldScreen,
}

#[derive(Debug, Clone)]
pub enum GuiMessage {
    StartingScreen(StartingScreenMessage),
    FieldScreen(FieldScreenMessage),
}

pub struct GameGui {
    starting_screen: StartingScreenController,
    field_screen: Option<FieldScreenController>,
    active_screen: ActiveScreen,
}

impl Application for GameGui {
    type Executor = executor::Default;
    type Message = GuiMessage;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let starting_screen = StartingScreenController::new();
        (
            Self {
                starting_screen,
                field_screen: None,
                active_screen: ActiveScreen::StartingScreen,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        let program_title = String::from("Game of Life");
        program_title
    }

    fn update(&mut self, message: GuiMessage) -> Command<GuiMessage> {
        match message {
            GuiMessage::StartingScreen(StartingScreenMessage::StartGame) => {
                self.active_screen = ActiveScreen::FieldScreen;
                self.field_screen = Some(FieldScreenController::new(
                    &self.starting_screen.x,
                    &self.starting_screen.y,
                ));
                Command::none()
            }
            GuiMessage::StartingScreen(message) => self
                .starting_screen
                .update(message)
                .map(GuiMessage::StartingScreen),
            GuiMessage::FieldScreen(message) => self
                .field_screen
                .as_mut()
                .expect("the field screen should not be empty")
                .update(message)
                .map(GuiMessage::FieldScreen),
        }
    }

    fn view(&self) -> iced::Element<'_, GuiMessage, Theme, Renderer> {
        match self.active_screen {
            ActiveScreen::StartingScreen => {
                self.starting_screen.view().map(GuiMessage::StartingScreen)
            }
            ActiveScreen::FieldScreen => self
                .field_screen
                .as_ref()
                .expect("the field screen should not be empty")
                .view()
                .map(GuiMessage::FieldScreen),
        }
    }

    fn subscription(&self) -> iced::Subscription<GuiMessage> {
        match self.active_screen {
            ActiveScreen::FieldScreen => iced::time::every(Duration::from_millis(200))
                .map(|_| GuiMessage::FieldScreen(FieldScreenMessage::Update)),
            _ => iced::Subscription::none(),
        }
    }
}
