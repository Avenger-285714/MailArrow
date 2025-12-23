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
    /// 
    /// # Arguments
    /// 
    /// * `url` - The server URL to normalize
    /// 
    /// # Returns
    /// 
    /// * `Ok(String)` - The normalized URL with proper scheme
    /// * `Err(String)` - Error message if URL is invalid
    /// 
    /// # Behavior
    /// 
    /// - Empty URLs are rejected
    /// - URLs with explicit http:// or https:// are preserved
    /// - localhost and 127.0.0.1 URLs default to http:// (for local testing)
    /// - All other URLs default to https://
    pub fn normalize_url(url: &str) -> Result<String, String> {
        let trimmed = url.trim();
        
        if trimmed.is_empty() {
            return Err("Server URL cannot be empty".to_string());
        }
        
        // If the URL already has a scheme, use it as-is
        if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
            return Ok(trimmed.to_string());
        }
        
        // Extract the host part (before any path or port)
        let host_part = trimmed.split('/').next().unwrap_or(trimmed);
        let host = host_part.split(':').next().unwrap_or(host_part);
        
        // Use http:// for localhost and 127.0.0.1 (local testing)
        // Use https:// for all other addresses (production servers)
        if host == "localhost" || host == "127.0.0.1" || host.starts_with("192.168.") || host.starts_with("10.") {
            Ok(format!("http://{}", trimmed))
        } else {
            Ok(format!("https://{}", trimmed))
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
