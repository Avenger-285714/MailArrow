mod eas;
mod ui;

use iced::{Application, Command, Element, Settings, Theme};
use ui::{LoginScreen, MainView};
use eas::{EasClient, Credentials};

fn main() -> iced::Result {
    env_logger::init();
    
    MailArrow::run(Settings {
        window: iced::window::Settings {
            size: iced::Size::new(1200.0, 800.0),
            min_size: Some(iced::Size::new(800.0, 600.0)),
            ..Default::default()
        },
        ..Default::default()
    })
}

#[derive(Debug)]
enum AppState {
    Login(LoginScreen),
    Main(MainView),
}

#[derive(Debug)]
struct MailArrow {
    state: AppState,
    eas_client: Option<EasClient>,
}

#[derive(Debug, Clone)]
enum Message {
    Login(ui::login::LoginMessage),
    MainView(ui::main_view::MainViewMessage),
    ConnectResult(Result<(), String>),
    FoldersLoaded(Result<Vec<eas::Folder>, String>),
    EmailsLoaded(Result<Vec<eas::Email>, String>),
}

impl Application for MailArrow {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                state: AppState::Login(LoginScreen::new()),
                eas_client: None,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        match &self.state {
            AppState::Login(_) => "MailArrow - Login".to_string(),
            AppState::Main(_) => "MailArrow".to_string(),
        }
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Login(login_msg) => {
                if let AppState::Login(login_screen) = &mut self.state {
                    match login_msg {
                        ui::login::LoginMessage::Connect => {
                            // Create credentials
                            let credentials = Credentials {
                                server_url: login_screen.server_url.clone(),
                                username: login_screen.username.clone(),
                                password: login_screen.password.clone(),
                                domain: if login_screen.domain.is_empty() {
                                    None
                                } else {
                                    Some(login_screen.domain.clone())
                                },
                            };
                            
                            // Create EAS client
                            let mut client = EasClient::new(credentials);
                            
                            // Store username for later
                            let username = login_screen.username.clone();
                            
                            // Connect asynchronously
                            return Command::perform(
                                async move {
                                    match client.connect().await {
                                        Ok(()) => {
                                            // Return both client and username on success
                                            Ok((client, username))
                                        }
                                        Err(e) => Err(e.to_string()),
                                    }
                                },
                                |result| {
                                    Message::ConnectResult(result.map(|_| ()))
                                },
                            );
                        }
                        _ => {
                            login_screen.update(login_msg);
                        }
                    }
                }
            }
            Message::MainView(main_msg) => {
                if let AppState::Main(main_view) = &mut self.state {
                    match main_msg.clone() {
                        ui::main_view::MainViewMessage::SelectFolder(folder_id) => {
                            main_view.update(main_msg);
                            
                            // Load emails for selected folder
                            if let Some(client) = &self.eas_client {
                                let client_clone = client.clone();
                                
                                return Command::perform(
                                    async move {
                                        client_clone.sync_emails(&folder_id).await
                                            .map_err(|e| e.to_string())
                                    },
                                    Message::EmailsLoaded,
                                );
                            }
                        }
                        ui::main_view::MainViewMessage::Refresh => {
                            // Refresh folders
                            if let Some(client) = &self.eas_client {
                                let client_clone = client.clone();
                                
                                return Command::perform(
                                    async move {
                                        client_clone.sync_folders().await
                                            .map_err(|e| e.to_string())
                                    },
                                    Message::FoldersLoaded,
                                );
                            }
                        }
                        ui::main_view::MainViewMessage::Logout => {
                            self.state = AppState::Login(LoginScreen::new());
                            self.eas_client = None;
                        }
                        _ => {
                            main_view.update(main_msg);
                        }
                    }
                }
            }
            Message::ConnectResult(result) => {
                match result {
                    Ok(()) => {
                        if let AppState::Login(login_screen) = &self.state {
                            // Create EAS client and store it
                            let credentials = Credentials {
                                server_url: login_screen.server_url.clone(),
                                username: login_screen.username.clone(),
                                password: login_screen.password.clone(),
                                domain: if login_screen.domain.is_empty() {
                                    None
                                } else {
                                    Some(login_screen.domain.clone())
                                },
                            };
                            
                            let mut client = EasClient::new(credentials);
                            let username = login_screen.username.clone();
                            
                            // Store client
                            self.eas_client = Some(client.clone());
                            
                            // Switch to main view
                            self.state = AppState::Main(MainView::new(username));
                            
                            // Load folders
                            return Command::perform(
                                async move {
                                    // Connect first
                                    if let Err(e) = client.connect().await {
                                        return Err(e.to_string());
                                    }
                                    // Then sync folders
                                    client.sync_folders().await
                                        .map_err(|e| e.to_string())
                                },
                                Message::FoldersLoaded,
                            );
                        }
                    }
                    Err(error) => {
                        if let AppState::Login(login_screen) = &mut self.state {
                            login_screen.error_message = Some(format!("Connection failed: {}", error));
                        }
                    }
                }
            }
            Message::FoldersLoaded(result) => {
                match result {
                    Ok(folders) => {
                        if let AppState::Main(main_view) = &mut self.state {
                            main_view.set_folders(folders);
                        }
                    }
                    Err(error) => {
                        log::error!("Failed to load folders: {}", error);
                    }
                }
            }
            Message::EmailsLoaded(result) => {
                match result {
                    Ok(emails) => {
                        if let AppState::Main(main_view) = &mut self.state {
                            main_view.set_emails(emails);
                        }
                    }
                    Err(error) => {
                        log::error!("Failed to load emails: {}", error);
                    }
                }
            }
        }
        
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        match &self.state {
            AppState::Login(login_screen) => {
                login_screen.view().map(Message::Login)
            }
            AppState::Main(main_view) => {
                main_view.view().map(Message::MainView)
            }
        }
    }
}
