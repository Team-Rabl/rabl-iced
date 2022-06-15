use iced::{button, Button, Column, Text, executor, Command, Application, Settings, Row, Container, container::{Style, StyleSheet}, Background, Length};

struct Counter {
    value: i32,
    inc_btn: button::State,
    dec_btn: button::State
}

impl Counter {
    pub fn view(&mut self) -> Column<Message> {
        Column::new()
            .push(Button::new(&mut self.inc_btn, Text::new("+"))
                .on_press(Message::IncPressed)
            )
            .push(Text::new(self.value.to_string()).size(50))
            .push(Button::new(&mut self.dec_btn, Text::new("-"))
                .on_press(Message::DecPressed)
            )
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::IncPressed => self.value += 1,
            Message::DecPressed => self.value -= 1
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncPressed,
    DecPressed
}

#[derive(Default)]
struct Rabl {
    value: i32,
    i: u64,
    inc_btn: button::State,
    dec_btn: button::State
}

impl Application for Rabl {
    type Executor = executor::Default;

    type Message = Message;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        // If needed to do some async on startunone())p, could provide a Command instead of Command::none()
        (
            Self::default(),
            Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("Rabl-iced (DEV BUILD)")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        self.i += 1;
        // Handles a message, update logic
        match message {
            Message::IncPressed => self.value += 1,
            Message::DecPressed => self.value -= 1
        }

        Command::none()
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        Container::new(
            Column::new()
                .spacing(20)
                .padding(20)
                .push(Text::new(String::from("Hello world!")))
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .style(ContainerStyle {})
            .into()
    }

    fn background_color(&self) -> iced::Color {
        iced::Color {
            r: cval(75),
            g: cval(75),
            b: cval(75),
            a: 1.0
        }
    }
}

pub fn cval(n: i32) -> f32 {
    n as f32 / 255.0
}

struct ContainerStyle {}
impl StyleSheet for ContainerStyle {
    fn style(&self) -> Style {
        let color = iced::Color {
            r: cval(75),
            g: cval(75),
            b: cval(75),
            a: 1.0
        };

        iced::container::Style {
            text_color: iced::Color::WHITE.into(),
            background: color.into(),
            ..iced::container::Style::default()
        }
    }
}

fn main() -> iced::Result {
    // No flags
    let app_settings = Settings {
        flags: (),
        ..Settings::default()
    };

    Rabl::run(app_settings)
}
