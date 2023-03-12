use iced::{
    executor, time,
    widget::{button, column as col, progress_bar, row, slider, text},
    window::{self, Position},
    Alignment, Application, Command, Element, Length, Settings, Subscription, Theme,
};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy)]
enum Message {
    Tick(Instant),
    SliderChanged(f32),
    ButtonPressed,
}

struct Timer {
    start: Instant,
    elapsed: Duration,
    max: Duration,
}

impl Timer {
    fn progress(&self) -> f32 {
        if self.max.as_secs_f32() < std::f32::EPSILON {
            1.
        } else {
            self.elapsed.as_secs_f32() / self.max.as_secs_f32()
        }
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self {
            start: Instant::now(),
            elapsed: Default::default(),
            max: Duration::from_secs(15),
        }
    }
}

impl Application for Timer {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Timer")
    }

    fn view(&self) -> Element<Self::Message> {
        col![
            row![
                text("Elapsed Time:").width(120),
                progress_bar(0.0..=1.0, self.progress())
            ]
            .align_items(Alignment::Center),
            row![
                text("").width(120),
                text(format!("{:.1} s", self.elapsed.as_secs_f32()))
            ],
            row![
                text("Duration:").width(120),
                slider(0.0..=30.0, self.max.as_secs_f32(), Message::SliderChanged).step(0.1),
            ],
            button("Reset Timer")
                .width(Length::Fill)
                .on_press(Message::ButtonPressed)
        ]
        .spacing(10)
        .padding(10)
        .into()
    }

    fn update(&mut self, msg: Self::Message) -> Command<Self::Message> {
        use Message::*;

        match msg {
            Tick(v) => {
                self.elapsed = (v - self.start).min(self.max);
            }
            SliderChanged(v) => {
                self.max = Duration::from_secs_f32(v);
            }
            ButtonPressed => {
                self.start = Instant::now();
                self.elapsed = Duration::default();
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_millis(10)).map(Message::Tick)
    }
}

fn main() -> iced::Result {
    Timer::run(Settings {
        window: window::Settings {
            size: (400, 150),
            position: Position::Centered,
            resizable: false,
            ..Default::default()
        },
        ..Default::default()
    })
}
