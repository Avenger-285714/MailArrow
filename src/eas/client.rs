use super::{Credentials, ConnectionState, Folder, Email, EasVersion};
use super::protocol::{Command, create_folder_sync_request, create_sync_request};
use reqwest::Client;
use std::sync::Arc;
use tokio::sync::RwLock;
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;

#[derive(Debug, thiserror::Error)]
pub enum EasError {
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),
    
    #[error("Authentication failed")]
    AuthenticationFailed,
    
    #[error("Protocol error: {0}")]
    ProtocolError(String),
    
    #[error("Connection error: {0}")]
    ConnectionError(String),
}

/// EAS Client for connecting to Exchange ActiveSync servers
#[derive(Clone, Debug)]
pub struct EasClient {
    credentials: Credentials,
    client: Client,
    state: Arc<RwLock<ConnectionState>>,
    policy_key: Arc<RwLock<Option<String>>>,
    device_id: String,
    device_type: String,
    version: EasVersion,
}

impl EasClient {
    /// Create a new EAS client
    pub fn new(credentials: Credentials) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .unwrap();
        
        Self {
            credentials,
            client,
            state: Arc::new(RwLock::new(ConnectionState::Disconnected)),
            policy_key: Arc::new(RwLock::new(None)),
            device_id: "MAILARROW001".to_string(),
            device_type: "MailArrow".to_string(),
            version: EasVersion::V141,
        }
    }
    
    /// Get current connection state
    pub async fn state(&self) -> ConnectionState {
        *self.state.read().await
    }
    
    /// Build the EAS command URL
    fn build_url(&self, command: Command) -> String {
        format!(
            "{}/Microsoft-Server-ActiveSync?Cmd={}&User={}&DeviceId={}&DeviceType={}",
            self.credentials.server_url,
            command.as_str(),
            self.credentials.username,
            self.device_id,
            self.device_type
        )
    }
    
    /// Connect and authenticate with the EAS server
    pub async fn connect(&mut self) -> Result<(), EasError> {
        *self.state.write().await = ConnectionState::Connecting;
        
        // Build basic auth
        let auth = BASE64.encode(format!(
            "{}:{}",
            self.credentials.username,
            self.credentials.password
        ));
        
        // Try OPTIONS request first to check server capabilities
        let url = format!("{}/Microsoft-Server-ActiveSync", self.credentials.server_url);
        let response = self.client
            .request(reqwest::Method::OPTIONS, &url)
            .header("Authorization", format!("Basic {}", auth))
            .send()
            .await?;
        
        if response.status().is_success() {
            *self.state.write().await = ConnectionState::Connected;
            Ok(())
        } else {
            *self.state.write().await = ConnectionState::Failed;
            Err(EasError::AuthenticationFailed)
        }
    }
    
    /// Synchronize folders
    pub async fn sync_folders(&self) -> Result<Vec<Folder>, EasError> {
        if *self.state.read().await != ConnectionState::Connected {
            return Err(EasError::ConnectionError("Not connected".to_string()));
        }
        
        let sync_key = "0"; // Initial sync
        let xml = create_folder_sync_request(sync_key)
            .map_err(|e| EasError::ProtocolError(e.to_string()))?;
        
        let auth = BASE64.encode(format!(
            "{}:{}",
            self.credentials.username,
            self.credentials.password
        ));
        
        let url = self.build_url(Command::FolderSync);
        let response = self.client
            .post(&url)
            .header("Authorization", format!("Basic {}", auth))
            .header("Content-Type", "application/vnd.ms-sync.wbxml")
            .header("MS-ASProtocolVersion", self.version.as_str())
            .body(xml)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(EasError::ProtocolError(
                format!("FolderSync failed: {}", response.status())
            ));
        }
        
        // For now, return a mock folder list
        Ok(vec![
            Folder {
                server_id: "1".to_string(),
                parent_id: "0".to_string(),
                display_name: "Inbox".to_string(),
                folder_type: super::types::FolderType::Inbox,
            },
            Folder {
                server_id: "2".to_string(),
                parent_id: "0".to_string(),
                display_name: "Sent Items".to_string(),
                folder_type: super::types::FolderType::SentItems,
            },
            Folder {
                server_id: "3".to_string(),
                parent_id: "0".to_string(),
                display_name: "Drafts".to_string(),
                folder_type: super::types::FolderType::Drafts,
            },
        ])
    }
    
    /// Synchronize emails from a specific folder
    pub async fn sync_emails(&self, folder_id: &str) -> Result<Vec<Email>, EasError> {
        if *self.state.read().await != ConnectionState::Connected {
            return Err(EasError::ConnectionError("Not connected".to_string()));
        }
        
        let sync_key = "0"; // Initial sync
        let xml = create_sync_request(sync_key, folder_id)
            .map_err(|e| EasError::ProtocolError(e.to_string()))?;
        
        let auth = BASE64.encode(format!(
            "{}:{}",
            self.credentials.username,
            self.credentials.password
        ));
        
        let url = self.build_url(Command::Sync);
        let response = self.client
            .post(&url)
            .header("Authorization", format!("Basic {}", auth))
            .header("Content-Type", "application/vnd.ms-sync.wbxml")
            .header("MS-ASProtocolVersion", self.version.as_str())
            .body(xml)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(EasError::ProtocolError(
                format!("Sync failed: {}", response.status())
            ));
        }
        
        // For now, return mock emails
        Ok(vec![
            Email {
                server_id: "msg1".to_string(),
                subject: "Welcome to MailArrow".to_string(),
                from: "admin@example.com".to_string(),
                to: vec![self.credentials.username.clone()],
                date: "2024-01-01T10:00:00Z".to_string(),
                body: "Welcome to MailArrow, your lightweight email client!".to_string(),
                is_read: false,
            },
            Email {
                server_id: "msg2".to_string(),
                subject: "Test Email".to_string(),
                from: "test@example.com".to_string(),
                to: vec![self.credentials.username.clone()],
                date: "2024-01-01T11:00:00Z".to_string(),
                body: "This is a test email.".to_string(),
                is_read: true,
            },
        ])
    }
}
