use iced::{
    widget::{
        container::{self},
        Container,
    },
    Color, Element, Theme,
};

use crate::messages::FieldMessages;

#[derive(PartialEq)]
pub enum Cell {
    Dead,
    Alive,
}

// pub fn show_cell_basic(c: &Cell) {
//     match c {
//         Cell::Alive => {
//             print!("#");
//         }
//         _ => {
//             print!("_")
//         }
//     }
// }

impl Cell {
    pub fn view(&self) -> Element<FieldMessages> {
        let color = match self {
            Cell::Alive => Color::new(39.0 / 255.0, 233.0 / 255.0, 52.0 / 255.0, 1.0),
            Cell::Dead => Color::new(76.0 / 255.0, 91.0 / 255.0, 101.0 / 255.0, 1.0),
        };

        let container = Container::new(iced::widget::Column::new())
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .style(move |_: &Theme| container::Appearance {
                background: Some(iced::Background::Color(color)),
                ..Default::default()
            });
        container.into()
    }
}
