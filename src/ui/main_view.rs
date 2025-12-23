use iced::widget::{button, column, container, row, scrollable, text, Column};
use iced::{Element, Length};
use crate::eas::{Email, Folder};

#[derive(Debug, Clone)]
pub enum MainViewMessage {
    SelectFolder(String),
    SelectEmail(String),
    Refresh,
    Logout,
}

#[derive(Debug)]
pub struct MainView {
    pub folders: Vec<Folder>,
    pub emails: Vec<Email>,
    pub selected_folder: Option<String>,
    pub selected_email: Option<String>,
    pub current_user: String,
}

impl MainView {
    pub fn new(current_user: String) -> Self {
        Self {
            folders: Vec::new(),
            emails: Vec::new(),
            selected_folder: None,
            selected_email: None,
            current_user,
        }
    }
    
    pub fn view(&self) -> Element<MainViewMessage> {
        let header = row![
            text(format!("MailArrow - {}", self.current_user))
                .size(24),
            button(text("Refresh"))
                .on_press(MainViewMessage::Refresh),
            button(text("Logout"))
                .on_press(MainViewMessage::Logout),
        ]
        .spacing(10)
        .padding(10);
        
        // Folder list
        let mut folder_list = Column::new().spacing(5);
        for folder in &self.folders {
            let is_selected = self.selected_folder.as_ref() == Some(&folder.server_id);
            let style = if is_selected {
                iced::theme::Button::Primary
            } else {
                iced::theme::Button::Secondary
            };
            
            folder_list = folder_list.push(
                button(text(&folder.display_name))
                    .on_press(MainViewMessage::SelectFolder(folder.server_id.clone()))
                    .width(Length::Fill)
                    .style(style)
            );
        }
        
        let folder_panel = container(scrollable(folder_list))
            .width(Length::FillPortion(2))
            .height(Length::Fill)
            .padding(10);
        
        // Email list
        let mut email_list = Column::new().spacing(5);
        for email in &self.emails {
            let is_selected = self.selected_email.as_ref() == Some(&email.server_id);
            let style = if is_selected {
                iced::theme::Button::Primary
            } else {
                iced::theme::Button::Secondary
            };
            
            let subject_text = if email.is_read {
                text(&email.subject)
            } else {
                text(format!("● {}", email.subject)).size(14)
            };
            
            let email_item = column![
                subject_text,
                text(&email.from).size(12),
                text(&email.date).size(10),
            ]
            .spacing(2);
            
            email_list = email_list.push(
                button(email_item)
                    .on_press(MainViewMessage::SelectEmail(email.server_id.clone()))
                    .width(Length::Fill)
                    .style(style)
            );
        }
        
        let email_list_panel = container(scrollable(email_list))
            .width(Length::FillPortion(3))
            .height(Length::Fill)
            .padding(10);
        
        // Email content
        let email_content = if let Some(selected_id) = &self.selected_email {
            if let Some(email) = self.emails.iter().find(|e| &e.server_id == selected_id) {
                column![
                    text(&email.subject).size(20),
                    text(format!("From: {}", email.from)).size(14),
                    text(format!("To: {}", email.to.join(", "))).size(14),
                    text(format!("Date: {}", email.date)).size(12),
                    container(text("")).height(20), // spacer
                    scrollable(text(&email.body)),
                ]
                .spacing(10)
                .padding(10)
            } else {
                column![text("Select an email to view")]
                    .padding(10)
            }
        } else {
            column![text("Select an email to view")]
                .padding(10)
        };
        
        let email_content_panel = container(email_content)
            .width(Length::FillPortion(5))
            .height(Length::Fill)
            .padding(10);
        
        // Main layout
        let main_content = row![
            folder_panel,
            email_list_panel,
            email_content_panel,
        ]
        .spacing(0)
        .height(Length::Fill);
        
        column![
            header,
            main_content,
        ]
        .into()
    }
    
    pub fn update(&mut self, message: MainViewMessage) {
        match message {
            MainViewMessage::SelectFolder(folder_id) => {
                self.selected_folder = Some(folder_id);
                self.selected_email = None;
            }
            MainViewMessage::SelectEmail(email_id) => {
                self.selected_email = Some(email_id);
            }
            MainViewMessage::Refresh => {
                // Refresh will be handled by the main app
            }
            MainViewMessage::Logout => {
                // Logout will be handled by the main app
            }
        }
    }
    
    pub fn set_folders(&mut self, folders: Vec<Folder>) {
        self.folders = folders;
    }
    
    pub fn set_emails(&mut self, emails: Vec<Email>) {
        self.emails = emails;
    }
}
