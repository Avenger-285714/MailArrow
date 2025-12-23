use iced::widget::{button, container, text, text_input, Column};
use iced::{Element, Length};

#[derive(Debug, Clone)]
pub enum LoginMessage {
    ServerUrlChanged(String),
    UsernameChanged(String),
    PasswordChanged(String),
    DomainChanged(String),
    Connect,
}

#[derive(Debug, Default)]
pub struct LoginScreen {
    pub server_url: String,
    pub username: String,
    pub password: String,
    pub domain: String,
    pub error_message: Option<String>,
}

impl LoginScreen {
    pub fn new() -> Self {
        Self {
            server_url: String::new(),
            username: String::new(),
            password: String::new(),
            domain: String::new(),
            error_message: None,
        }
    }
    
    pub fn view(&self) -> Element<LoginMessage> {
        let title = text("MailArrow - EAS Email Client")
            .size(32);
        
        let server_input = text_input("Server URL (e.g., mail.example.com or https://mail.example.com)", &self.server_url)
            .on_input(LoginMessage::ServerUrlChanged)
            .padding(10);
        
        let username_input = text_input("Username", &self.username)
            .on_input(LoginMessage::UsernameChanged)
            .padding(10);
        
        let password_input = text_input("Password", &self.password)
            .on_input(LoginMessage::PasswordChanged)
            .secure(true)
            .padding(10);
        
        let domain_input = text_input("Domain (optional)", &self.domain)
            .on_input(LoginMessage::DomainChanged)
            .padding(10);
        
        let connect_button = button(
            text("Connect")
                .horizontal_alignment(iced::alignment::Horizontal::Center)
        )
        .on_press(LoginMessage::Connect)
        .padding(10);
        
        let mut content = Column::new()
            .spacing(20)
            .padding(40)
            .max_width(500)
            .push(title)
            .push(server_input)
            .push(username_input)
            .push(password_input)
            .push(domain_input)
            .push(connect_button);
        
        if let Some(error) = &self.error_message {
            content = content.push(
                text(error)
                    .style(iced::theme::Text::Color(iced::Color::from_rgb(0.8, 0.0, 0.0)))
            );
        }
        
        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
    
    pub fn update(&mut self, message: LoginMessage) {
        match message {
            LoginMessage::ServerUrlChanged(url) => {
                self.server_url = url;
            }
            LoginMessage::UsernameChanged(username) => {
                self.username = username;
            }
            LoginMessage::PasswordChanged(password) => {
                self.password = password;
            }
            LoginMessage::DomainChanged(domain) => {
                self.domain = domain;
            }
            LoginMessage::Connect => {
                // Connection will be handled by the main app
            }
        }
    }
}
