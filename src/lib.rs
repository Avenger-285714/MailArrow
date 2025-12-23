/// MailArrow - Lightweight Email Client with EAS Protocol Support
/// 
/// This library provides the core functionality for the MailArrow email client,
/// including EAS (Exchange ActiveSync) protocol implementation and UI components.

pub mod eas;
pub mod ui;

// Re-export commonly used types
pub use eas::{EasClient, Credentials, Email, Folder, ConnectionState};
