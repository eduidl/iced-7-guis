use iced::{
    theme::TextInput,
    widget::{row, text, text_input},
    window::{self, Position},
    Alignment, Element, Sandbox, Settings,
};
use iced_7_guis::TextInputValidateion;

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

    fn view(&self) -> Element<Self::Message> {
        let celcius_style = TextInput::Custom(Box::new(TextInputValidateion(validate_input(
            &self.celcius,
        ))));

        let fahrenheit_style = TextInput::Custom(Box::new(TextInputValidateion(validate_input(
            &self.fahrenheit,
        ))));

        row![
            text_input("", &self.celcius, Message::Celcius)
                .width(200)
                .style(celcius_style),
            text("Celius ="),
            text_input("", &self.fahrenheit, Message::Fahrenheit)
                .width(200)
                .style(fahrenheit_style),
            text("Fahrenheit"),
        ]
        .spacing(10)
        .padding(10)
        .align_items(Alignment::Center)
        .into()
    }

    fn update(&mut self, message: Self::Message) {
        use Message::*;

        match message {
            Celcius(v) => {
                self.celcius = v;
                if let Ok(c) = self.celcius.parse::<f32>() {
                    self.fahrenheit = celcius_to_fahrenheit(c).round().to_string()
                }
            }
            Fahrenheit(v) => {
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
