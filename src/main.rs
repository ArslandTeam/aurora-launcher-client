use iced::{Element, Length, Size, widget::container, window::Settings};

use crate::{
    config::CONFIG,
    pages::authentication::{AuthenticationMessage, AuthenticationState},
};
mod api;
mod config;
mod pages;

#[derive(Debug, Clone)]
pub enum Message {
    Authentication(AuthenticationMessage),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Page {
    Authentication,
}

pub struct App {
    pub current_page: Page,
    pub auth: AuthenticationState,
}

impl Default for App {
    fn default() -> Self {
        Self {
            current_page: Page::Authentication,
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
            Page::Authentication => {
                crate::pages::authentication::AuthenticationState::view(&self.auth)
                    .map(Message::Authentication)
            }
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
            id: Some(CONFIG.app_settings.id.clone()),
            ..Default::default()
        })
        .window(Settings {
            size: Size {
                width: CONFIG.window_settings.width,
                height: CONFIG.window_settings.height,
            },
            ..Default::default()
        })
        .run()
}
