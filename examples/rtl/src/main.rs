use iced::widget::{center, row, toggler};
use iced::{Direction, Element, Length, Settings};

pub fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .title("RTL Example")
        .settings(Settings {
            default_direction: Direction::RightToLeft,
            ..Default::default()
        })
        .run()
}

#[derive(Default)]
struct App {
    value: bool,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Toggled(bool),
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::Toggled(value) => {
                self.value = value;
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let rtl_text = toggler(self.value)
            .label("فارسی")
            .on_toggle(Message::Toggled);

        let ltr_text = toggler(self.value)
            .label("English")
            .on_toggle(Message::Toggled);

        let content = row![rtl_text, ltr_text].spacing(20).width(Length::Fill);

        center(content).into()
    }
}
