use iced::{
    alignment::Horizontal,
    widget::{button, row, text},
    window::{self, Position},
    Element, Sandbox, Settings,
};

#[derive(Debug, Clone, Copy)]
struct Increment;

#[derive(Default)]
struct Counter {
    value: i32,
}

impl Sandbox for Counter {
    type Message = Increment;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Counter")
    }

    fn view(&self) -> Element<Self::Message> {
        row![
            text(self.value)
                .size(30)
                .width(80)
                .horizontal_alignment(Horizontal::Right),
            button("Count").on_press(Increment),
        ]
        .spacing(10)
        .padding(10)
        .into()
    }

    fn update(&mut self, _: Self::Message) {
        self.value += 1;
    }
}

fn main() -> iced::Result {
    Counter::run(Settings {
        window: window::Settings {
            size: (180, 50),
            position: Position::Centered,
            resizable: false,
            ..Default::default()
        },
        ..Default::default()
    })
}
