use iced::{
    executor, multi_window,
    time::{self, Duration},
    window, Application, Command, Renderer, Size, Theme,
};

use crate::{
    field_screen::{self, ActiveStatus, FieldScreenController, FieldScreenMessage},
    start_screen::{StartingScreenController, StartingScreenMessage},
    stop_warning_window::{self, StopWarningWindow, StopWarningWindowMessage},
};

#[derive(PartialEq)]
enum ActiveScreen {
    StartingScreen,
    FieldScreen,
}

#[derive(Debug, Clone)]
pub enum GuiMessage {
    StartingScreen(StartingScreenMessage),
    FieldScreen(FieldScreenMessage),
    StopWarningWindow(StopWarningWindowMessage),
}

pub struct GameGui {
    starting_screen: StartingScreenController,
    field_screen: Option<FieldScreenController>,
    stop_warning_window: Option<StopWarningWindow>,
    active_screen: ActiveScreen,
}

impl multi_window::Application for GameGui {
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
                stop_warning_window: None,
            },
            Command::none(),
        )
    }

    fn title(&self, _window: window::Id) -> String {
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
            GuiMessage::FieldScreen(FieldScreenMessage::DisplayWarningWindow) => {
                // self.field_screen
                //     .as_mut()
                //     .expect("the field screen should not be empty")
                //     .update(FieldScreenMessage::DisplayWarningWindow)
                //     .map(GuiMessage::FieldScreen);
                self.stop_warning_window = Some(StopWarningWindow::new());
                let (id, window) = window::spawn(window::Settings {
                    size: Size::new(300.0, 200.0),
                    position: window::Position::Centered,
                    visible: true,
                    resizable: false,
                    level: window::Level::AlwaysOnTop,
                    ..Default::default()
                });
                self.stop_warning_window.as_mut().unwrap().window_id = id;
                window
            }
            GuiMessage::FieldScreen(message) => self
                .field_screen
                .as_mut()
                .expect("the field screen should not be empty")
                .update(message)
                .map(GuiMessage::FieldScreen),
            GuiMessage::StopWarningWindow(message) => {
                self.stop_warning_window
                    .as_mut()
                    .expect("the warning window should not be empty")
                    .update(message);
                Command::none()
            }
        }
    }

    fn view(&self, window_id: window::Id) -> iced::Element<'_, GuiMessage, Theme, Renderer> {
        if self
            .stop_warning_window
            .as_ref()
            .map_or(false, |w| w.window_id == window_id)
        {
            self.stop_warning_window
                .as_ref()
                .unwrap()
                .view()
                .map(GuiMessage::StopWarningWindow)
        } else {
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
    }

    fn subscription(&self) -> iced::Subscription<GuiMessage> {
        if self.active_screen == ActiveScreen::FieldScreen {
            match self.field_screen.as_ref().unwrap().active_status {
                ActiveStatus::Paused => iced::Subscription::none(),
                ActiveStatus::Playing => time::every(Duration::from_millis(200))
                    .map(|_| GuiMessage::FieldScreen(FieldScreenMessage::Update)),
            }
        } else {
            iced::Subscription::none()
        }
    }
}
