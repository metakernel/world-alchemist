use iced::{Element, Task, widget::Text};

struct WorldAlchemistApp;

pub type WorldAlchemistAppResult = iced::Result;

pub fn start_app() -> WorldAlchemistAppResult {
    iced::application(WorldAlchemistApp::new,WorldAlchemistApp::update,WorldAlchemistApp::view)
    // .theme(crate::theme::get_theme())
    // .font(crate::font::get_font())
    // .default_font(crate::font::get_default_ font())
    .run()
}

pub enum Message {
    // Define your application messages here
}

impl WorldAlchemistApp {
    fn new() -> Self {
        WorldAlchemistApp
    }

    fn update(&mut self, _message: Message) -> Task<Message> {
        Task::none()
    }

    fn view(&self) -> Element<'_,Message> {
        // Your UI code here
        Text::new("Hello, World!").into()
    }
}