use iced::{
    widget::{row, text, text_input},
    window::{self, Position},
    Alignment, Element, Sandbox, Settings,
};
use iced_7_guis::Input;

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

struct TempConv {
    celcius: Input,
    fahrenheit: Input,
}

impl Sandbox for TempConv {
    type Message = Message;

    fn new() -> Self {
        Self {
            celcius: Input::Valid("".into()),
            fahrenheit: Input::Valid("".into()),
        }
    }

    fn title(&self) -> String {
        String::from("TempConv")
    }

    fn view(&self) -> Element<Self::Message> {
        row![
            text_input("", self.celcius.as_str(), Message::Celcius)
                .width(200)
                .style(self.celcius.style()),
            text("Celius ="),
            text_input("", self.fahrenheit.as_str(), Message::Fahrenheit)
                .width(200)
                .style(self.fahrenheit.style()),
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
                if let Ok(c) = v.parse::<f32>() {
                    self.celcius = Input::Valid(v);
                    self.fahrenheit = Input::Valid(celcius_to_fahrenheit(c).round().to_string());
                } else {
                    self.celcius = Input::Invalid(v);
                }
            }
            Fahrenheit(v) => {
                if let Ok(f) = v.parse::<f32>() {
                    self.fahrenheit = Input::Valid(v);
                    self.celcius = Input::Valid(fahrenheit_to_celcius(f).round().to_string());
                } else {
                    self.fahrenheit = Input::Invalid(v);
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
