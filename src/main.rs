use iced::{Column, Text, executor, Command, Application, Settings, Container, Length};

use rabl_iced::styles;

#[derive(Default)]
struct Rabl;

impl Application for Rabl {
    type Executor = executor::Default;

    type Message = ();

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        // If needed to do some async on startunone())p, could provide a Command instead of Command::none()
        ( Self::default(), Command::none() )
    }

    fn title(&self) -> String {
        String::from("Rabl-iced (DEV BUILD)")
    }

    fn update(&mut self, _message: Self::Message) -> iced::Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        // App content
        let app_content = Column::new()
            .spacing(20)
            .padding(20)
            .push(Text::new(String::from("Rabl Login")))
            .push(Text::new(String::from("Username:")))
            .push(Text::new(String::from("Password:")));

        // Main app container
        Container::new(app_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(styles::Container)
            .center_x()
            .into()
    }

    fn background_color(&self) -> iced::Color {
        styles::colors::BACKGROUND
    }
}

fn main() -> iced::Result {
    // No flags
    let app_settings = Settings {
        ..Settings::default()
    };

    Rabl::run(app_settings)
}
