use iced::{
    border::Radius,
    theme::{self, Custom, Palette, Theme},
    widget::button::Appearance,
    Border, Color,
};

// pub enum ThemeChoice {
//     LightTheme,
//     DarkTheme,
// }

#[derive(Debug, Clone, Default)]
pub struct CustomTheme {}

impl CustomTheme {
    const PALETTE: Palette = Palette {
        background: Color::from_rgb(1.0, 22.0, 39.0),
        text: Color::from_rgb(253.0, 255.0, 252.0),
        primary: Color::from_rgb(255.0, 159.0, 28.0),
        success: Color::from_rgb(46.0, 196.0, 182.0),
        danger: Color::from_rgb(231.0, 29.0, 54.0),
    };
    const BUTTON_BORDER_RADIUS: f32 = 1.0;
    const BUTTON_BORDER_WIDTH: f32 = 3.0;
    const HOVER_BUTTON_COLOR: Color = Color::from_rgb(240.0, 149.0, 26.0);
    pub fn new() -> Custom {
        Custom::new(String::from("Dark theme"), CustomTheme::PALETTE)
    }
}

impl iced::widget::button::StyleSheet for CustomTheme {
    type Style = ();
    fn active(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
        iced::widget::button::Appearance {
            background: Some(iced::Background::Color(CustomTheme::PALETTE.primary)),
            border: Border {
                color: CustomTheme::PALETTE.text,
                width: CustomTheme::BUTTON_BORDER_WIDTH,
                radius: Radius::from(CustomTheme::BUTTON_BORDER_RADIUS),
            },
            text_color: CustomTheme::PALETTE.text,
            ..Default::default()
        }
    }

    fn hovered(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
        iced::widget::button::Appearance {
            background: Some(iced::Background::Color(CustomTheme::HOVER_BUTTON_COLOR)),
            border: Border {
                color: CustomTheme::PALETTE.text,
                width: CustomTheme::BUTTON_BORDER_WIDTH,
                radius: Radius::from(CustomTheme::BUTTON_BORDER_RADIUS),
            },
            text_color: CustomTheme::PALETTE.text,
            ..Default::default()
        }
    }
}
