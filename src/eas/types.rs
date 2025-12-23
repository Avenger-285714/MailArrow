use serde::{Deserialize, Serialize};

/// EAS protocol version
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EasVersion {
    V141,
    V160,
}

impl EasVersion {
    pub fn as_str(&self) -> &'static str {
        match self {
            EasVersion::V141 => "14.1",
            EasVersion::V160 => "16.0",
        }
    }
}

/// EAS folder types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FolderType {
    Inbox,
    Drafts,
    DeletedItems,
    SentItems,
    Outbox,
    Calendar,
    Contacts,
    Tasks,
    Notes,
    UserCreatedFolder,
}

/// Represents an EAS folder
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Folder {
    pub server_id: String,
    pub parent_id: String,
    pub display_name: String,
    pub folder_type: FolderType,
}

/// Represents an email message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Email {
    pub server_id: String,
    pub subject: String,
    pub from: String,
    pub to: Vec<String>,
    pub date: String,
    pub body: String,
    pub is_read: bool,
}

/// EAS credentials
#[derive(Debug, Clone)]
pub struct Credentials {
    pub server_url: String,
    pub username: String,
    pub password: String,
    pub domain: Option<String>,
}

/// EAS connection state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Failed,
}
