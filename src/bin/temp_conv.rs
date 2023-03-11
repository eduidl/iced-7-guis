use iced::{
    theme::TextInput,
    widget::{row, text, text_input},
    window::{self, Position},
    Alignment, Color, Element, Sandbox, Settings, Theme,
};

fn validate_input(degree: &str) -> bool {
    degree.is_empty() || degree.parse::<f32>().is_ok()
}

fn celcius_to_fahrenheit(degree: f32) -> f32 {
    degree * (9. / 5.) + 32.
}

fn fahrenheit_to_celcius(degree: f32) -> f32 {
    (degree - 32.) * (5. / 9.)
}

#[derive(Debug, Clone)]
enum Message {
    Celcius(String),
    Fahrenheit(String),
}

#[derive(Debug, Clone, Copy)]
struct TextInputValidateion(bool);

impl text_input::StyleSheet for TextInputValidateion {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> text_input::Appearance {
        let palette = style.extended_palette();

        text_input::Appearance {
            background: if self.0 {
                palette.background.base
            } else {
                palette.danger.base
            }
            .color
            .into(),
            border_radius: 2.,
            border_width: 1.,
            border_color: palette.background.strong.color,
        }
    }

    fn hovered(&self, style: &Self::Style) -> text_input::Appearance {
        let palette = style.extended_palette();

        text_input::Appearance {
            background: if self.0 {
                palette.background.base
            } else {
                palette.danger.base
            }
            .color
            .into(),
            border_radius: 2.,
            border_width: 1.,
            border_color: palette.background.base.text,
        }
    }

    fn focused(&self, style: &Self::Style) -> text_input::Appearance {
        let palette = style.extended_palette();

        text_input::Appearance {
            background: if self.0 {
                palette.background.base
            } else {
                palette.danger.base
            }
            .color
            .into(),
            border_radius: 2.,
            border_width: 1.,
            border_color: palette.primary.strong.color,
        }
    }

    fn placeholder_color(&self, style: &Self::Style) -> Color {
        style.extended_palette().background.strong.color
    }

    fn value_color(&self, style: &Self::Style) -> Color {
        style.extended_palette().background.base.text
    }

    fn selection_color(&self, style: &Self::Style) -> Color {
        style.extended_palette().primary.weak.color
    }
}

#[derive(Default)]
struct TempConv {
    celcius: String,
    fahrenheit: String,
}

impl Sandbox for TempConv {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("TempConv")
    }

    fn view(&self) -> Element<Message> {
        let celcius_style = TextInput::Custom(Box::new(TextInputValidateion(validate_input(
            &self.celcius,
        ))));

        let fahrenheit_style = TextInput::Custom(Box::new(TextInputValidateion(validate_input(
            &self.fahrenheit,
        ))));

        row![
            text_input("", &self.celcius, Message::Celcius)
                .width(200)
                .ristyle(celcius_style),
            text("Celius ="),
            text_input("", &self.fahrenheit, Message::Fahrenheit)
                .width(200)
                .ristyle(fahrenheit_style),
            text("Fahrenheit"),
        ]
        .spacing(10)
        .padding(10)
        .align_items(Alignment::Center)
        .into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Celcius(v) => {
                self.celcius = v;
                if let Ok(c) = self.celcius.parse::<f32>() {
                    self.fahrenheit = celcius_to_fahrenheit(c).round().to_string()
                }
            }
            Message::Fahrenheit(v) => {
                self.fahrenheit = v;
                if let Ok(f) = self.fahrenheit.parse::<f32>() {
                    self.celcius = fahrenheit_to_celcius(f).round().to_string()
                }
            }
        }
    }
}

fn main() -> iced::Result {
    TempConv::run(Settings {
        window: window::Settings {
            size: (600, 50),
            position: Position::Centered,
            resizable: false,
            ..Default::default()
        },
        ..Default::default()
    })
}
