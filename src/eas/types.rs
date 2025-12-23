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

impl Credentials {
    /// Validate and normalize the server URL
    pub fn normalize_url(url: &str) -> Result<String, String> {
        let trimmed = url.trim();
        
        if trimmed.is_empty() {
            return Err("Server URL cannot be empty".to_string());
        }
        
        // If the URL doesn't start with http:// or https://, add https://
        if !trimmed.starts_with("http://") && !trimmed.starts_with("https://") {
            Ok(format!("https://{}", trimmed))
        } else {
            Ok(trimmed.to_string())
        }
    }
}

/// EAS connection state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Failed,
}
