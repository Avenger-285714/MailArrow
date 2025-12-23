use iced::widget::{button, container, row, text, text_input, Column, Row};
use iced::{Element, Length};

#[derive(Debug, Clone)]
pub enum LoginMessage {
    ServerUrlChanged(String),
    UsernameChanged(String),
    PasswordChanged(String),
    DomainChanged(String),
    TogglePasswordVisibility,
    Connect,
}

#[derive(Debug, Default)]
pub struct LoginScreen {
    pub server_url: String,
    pub username: String,
    pub password: String,
    pub domain: String,
    pub error_message: Option<String>,
    pub show_password: bool,
}

impl LoginScreen {
    pub fn new() -> Self {
        Self {
            server_url: String::new(),
            username: String::new(),
            password: String::new(),
            domain: String::new(),
            error_message: None,
            show_password: false,
        }
    }
    
    pub fn view(&self) -> Element<LoginMessage> {
        let title = text("MailArrow - EAS Email Client")
            .size(32);
        
        let server_input = text_input("Server URL (e.g., mail.example.com, 127.0.0.1:8080)", &self.server_url)
            .on_input(LoginMessage::ServerUrlChanged)
            .padding(10);
        
        let username_input = text_input("Username", &self.username)
            .on_input(LoginMessage::UsernameChanged)
            .padding(10);
        
        // Password input with visibility toggle
        let password_input = text_input("Password", &self.password)
            .on_input(LoginMessage::PasswordChanged)
            .secure(!self.show_password)
            .padding(10);
        
        let toggle_icon = if self.show_password { "👁" } else { "👁‍🗨" };
        let toggle_button = button(text(toggle_icon).size(20))
            .on_press(LoginMessage::TogglePasswordVisibility)
            .padding(10);
        
        let password_row = Row::new()
            .spacing(10)
            .push(password_input)
            .push(toggle_button);
        
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
            .push(password_row)
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
            LoginMessage::TogglePasswordVisibility => {
                self.show_password = !self.show_password;
            }
            LoginMessage::Connect => {
                // Connection will be handled by the main app
            }
        }
    }
}
