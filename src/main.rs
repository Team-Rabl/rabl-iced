use iced::{Column, Text, executor, Command, Application, Settings, Container, Length, TextInput};

use rabl_iced::styles;

// Messages must derive Clone
#[derive(Debug, Clone)]
enum Message {
    TextInputChanged(String)
}

#[derive(Default)]
struct Rabl {
    username: iced::text_input::State,
    usr_val: String,
    // password: iced::text_input::State,
    // p_val: String
}

impl Application for Rabl {
    type Executor = executor::Default;

    type Message = Message;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        // If needed to do some async on startunone())p, could provide a Command instead of Command::none()
        ( Self::default(), Command::none() )
    }

    fn title(&self) -> String {
        String::from("Rabl-iced (DEV BUILD)")
    }

    fn update(&mut self, _message: Self::Message) -> iced::Command<Self::Message> {
        // println!("{:?}", _message);
        match _message {
            Message::TextInputChanged(s) => {
                self.usr_val = s
            },
        };

        Command::none()
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let inp = TextInput::new(
            &mut self.username, 
            "Username", 
            &self.usr_val, 
            Message::TextInputChanged
        );

        
        // App content
        let app_content = Column::new()
            .spacing(20)
            .padding(20)
            .push(Text::new(String::from("Rabl Login")))
            .push(
                iced::Row::new()
                .push(Text::new(String::from("Username:")))
                .push(inp)
            )
            .push(
                iced::Row::new()
                .push(Text::new(String::from("Password:")))
            );

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
