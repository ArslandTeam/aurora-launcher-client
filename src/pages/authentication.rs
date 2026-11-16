use iced::{
    Element, Length,
    widget::{button, column, container, text, text_input},
};

#[derive(Debug, Default)]
pub struct AuthenticationState {
    pub username: String,
    pub password: String,
    pub is_loading: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone)]
pub enum AuthenticationMessage {
    UsernameChanged(String),
    PasswordChanged(String),
    Login,
    LoginSuccess,
    LoginFailed(String),
}

impl AuthenticationState {
    pub fn update(&mut self, message: AuthenticationMessage) {
        match message {
            AuthenticationMessage::UsernameChanged(value) => {
                self.username = value;
                self.error = None;
            }

            AuthenticationMessage::PasswordChanged(value) => {
                self.password = value;
                self.error = None;
            }

            AuthenticationMessage::Login => {
                if self.username.is_empty() || self.password.is_empty() {
                    self.error = Some("Заполните все поля".to_string());
                    return;
                }

                self.is_loading = true;
            }

            AuthenticationMessage::LoginSuccess => {
                self.is_loading = false;
            }

            AuthenticationMessage::LoginFailed(error) => {
                self.is_loading = false;
                self.error = Some(error);
            }
        }
    }

    pub fn view(state: &AuthenticationState) -> Element<'_, AuthenticationMessage> {
        let username = text_input("Имя пользователя", &state.username)
            .on_input(AuthenticationMessage::UsernameChanged)
            .padding(10);

        let password = text_input("Пароль", &state.password)
            .secure(true)
            .on_input(AuthenticationMessage::PasswordChanged)
            .padding(10);

        let login_button = button(text(if state.is_loading {
            "Вход..."
        } else {
            "Войти"
        }))
        .on_press(AuthenticationMessage::Login)
        .padding(10);

        let mut content = column![
            text!("Авторизация").size(32),
            username,
            password,
            login_button,
        ]
        .spacing(15)
        .width(Length::Fixed(300.0));

        if let Some(error) = &state.error {
            content = content.push(text(error));
        }

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }
}
