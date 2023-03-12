use chrono::{Local, NaiveDate};
use iced::{
    theme::TextInput,
    widget::{button, column as col, pick_list, text, text_input},
    window::{self, Position},
    Element, Length, Sandbox, Settings,
};
use iced_7_guis::TextInputValidateion;
use regex::Regex;

const DATE_FORMAT: &str = "%d.%m.%Y";

fn validate_date_str(date: &str) -> bool {
    let re = Regex::new(r"^\d{2}\.\d{2}\.\d{4}$").unwrap();

    re.find(date).is_some() && NaiveDate::parse_from_str(date, DATE_FORMAT).is_ok()
}

#[derive(Debug, Clone)]
enum Message {
    FlightTypeSelected(FlightType),
    FromTextInput(String),
    ToTextInput(String),
    ButtonPressed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FlightType {
    OneWay,
    Return,
}

impl FlightType {
    const ALL: [Self; 2] = [Self::OneWay, Self::Return];
}

impl Default for FlightType {
    fn default() -> Self {
        Self::OneWay
    }
}

impl std::fmt::Display for FlightType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::OneWay => "one-way flight",
                Self::Return => "return flight",
            }
        )
    }
}

#[derive(Debug, Clone)]
struct Book {
    flight_type: FlightType,
    from: String,
    to: String,
}

impl Book {
    fn msg(&self) -> String {
        format!(
            "You've booked a {} from {} to {}",
            self.flight_type, self.from, self.to
        )
    }
}

struct FlightBooker {
    flight_type: Option<FlightType>,
    from: String,
    to: String,
    booked: Option<Book>,
}

impl Sandbox for FlightBooker {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Filght Booker")
    }

    fn view(&self) -> Element<Self::Message> {
        let from_valid = validate_date_str(&self.from);
        let to_valid = validate_date_str(&self.to);

        let mut button = button("Book").width(Length::Fill);
        if self.flight_type.is_some() && from_valid && to_valid {
            button = button.on_press(Message::ButtonPressed);
        };

        let mut layout = col![
            pick_list(
                &FlightType::ALL[..],
                self.flight_type,
                Message::FlightTypeSelected
            )
            .width(Length::Fill),
            text_input("", &self.from, Message::FromTextInput).style(TextInput::Custom(Box::new(
                TextInputValidateion(from_valid)
            ))),
            text_input("", &self.to, Message::ToTextInput)
                .style(TextInput::Custom(Box::new(TextInputValidateion(to_valid)))),
            button
        ]
        .spacing(10)
        .padding(10);

        if let Some(book) = &self.booked {
            layout = layout.push(text(book.msg()));
        }

        layout.into()
    }

    fn update(&mut self, m: Self::Message) {
        use Message::*;

        match m {
            FlightTypeSelected(v) => {
                self.flight_type = Some(v);
            }
            FromTextInput(v) => {
                self.from = v;
            }
            ToTextInput(v) => {
                self.to = v;
            }
            ButtonPressed => {
                self.booked = Some(Book {
                    flight_type: self.flight_type.unwrap(),
                    from: self.from.clone(),
                    to: self.to.clone(),
                });
            }
        }
    }
}

impl Default for FlightBooker {
    fn default() -> Self {
        let now = Local::now();
        let date_str = now.date_naive().format(DATE_FORMAT).to_string();

        Self {
            flight_type: Some(FlightType::default()),
            from: date_str.clone(),
            to: date_str,
            booked: None,
        }
    }
}

fn main() -> iced::Result {
    FlightBooker::run(Settings {
        window: window::Settings {
            size: (300, 250),
            position: Position::Centered,
            // resizable: false,
            ..Default::default()
        },
        ..Default::default()
    })
}
