use iced::{Element, Length, Size, widget::container, window::Settings};

use crate::pages::authentication::{AuthenticationMessage, AuthenticationState};
mod pages;

#[derive(Debug, Clone)]
pub enum Message {
    Authentication(AuthenticationMessage),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Page {
    Auth,
}

pub struct App {
    pub current_page: Page,
    pub auth: AuthenticationState,
}

impl Default for App {
    fn default() -> Self {
        Self {
            current_page: Page::Auth,
            auth: AuthenticationState::default(),
        }
    }
}

impl App {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Authentication(message) => {
                self.auth.update(message);
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let content = match self.current_page {
            Page::Auth => crate::pages::authentication::AuthenticationState::view(&self.auth)
                .map(Message::Authentication),
        };

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        // TODO создать функцию для чтения конфига
        .settings(iced::Settings {
            id: Some("aurora-launcher".to_string()),
            ..Default::default()
        })
        .window(Settings {
            size: Size {
                width: 900.0,
                height: 550.0,
            },
            ..Default::default()
        })
        .run()
}
