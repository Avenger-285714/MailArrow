# EAS Protocol API Documentation

## Overview

This document describes the EAS (Exchange ActiveSync) protocol implementation in MailArrow.

## EAS Client API

### Creating a Client

```rust
use mailarrow::eas::{EasClient, Credentials};

let credentials = Credentials {
    server_url: "https://mail.example.com".to_string(),
    username: "user@example.com".to_string(),
    password: "password".to_string(),
    domain: None,
};

let client = EasClient::new(credentials);
```

### Connecting to Server

```rust
// Connect performs authentication
match client.connect().await {
    Ok(()) => println!("Connected successfully"),
    Err(e) => eprintln!("Connection failed: {}", e),
}
```

### Syncing Folders

```rust
let folders = client.sync_folders().await?;

for folder in folders {
    println!("{}: {}", folder.display_name, folder.server_id);
}
```

### Syncing Emails

```rust
let emails = client.sync_emails("inbox_id").await?;

for email in emails {
    println!("From: {}, Subject: {}", email.from, email.subject);
}
```

## Data Structures

### Credentials

```rust
pub struct Credentials {
    pub server_url: String,      // EAS server URL (with https://)
    pub username: String,        // Email username
    pub password: String,        // Email password  
    pub domain: Option<String>,  // Windows domain (optional)
}
```

**Methods**:
- `normalize_url(url: &str) -> Result<String, String>`: Validates and normalizes server URL

### Email

```rust
pub struct Email {
    pub server_id: String,    // Unique identifier from server
    pub subject: String,      // Email subject line
    pub from: String,        // Sender email address
    pub to: Vec<String>,     // Recipient email addresses
    pub date: String,        // Date sent (ISO 8601 format)
    pub body: String,        // Email body (plain text)
    pub is_read: bool,       // Whether email has been read
}
```

### Folder

```rust
pub struct Folder {
    pub server_id: String,       // Unique identifier from server
    pub parent_id: String,       // Parent folder ID (or "0" for root)
    pub display_name: String,    // Folder name shown to user
    pub folder_type: FolderType, // Type of folder
}
```

### FolderType

```rust
pub enum FolderType {
    Inbox,              // Main inbox
    Drafts,             // Draft messages
    DeletedItems,       // Trash/deleted items
    SentItems,          // Sent messages
    Outbox,             // Outgoing messages
    Calendar,           // Calendar items
    Contacts,           // Contact information
    Tasks,              // Task items
    Notes,              // Note items
    UserCreatedFolder,  // Custom user folder
}
```

### ConnectionState

```rust
pub enum ConnectionState {
    Disconnected,  // Not connected
    Connecting,    // Connection in progress
    Connected,     // Successfully connected
    Failed,        // Connection failed
}
```

### EasError

```rust
pub enum EasError {
    HttpError(reqwest::Error),   // HTTP/network error
    AuthenticationFailed,         // Authentication failed (401/403)
    ProtocolError(String),        // EAS protocol error
    ConnectionError(String),      // Generic connection error
}
```

## EAS Protocol Commands

### FolderSync Command

**Purpose**: Synchronize folder hierarchy

**Request XML**:
```xml
<?xml version="1.0" encoding="utf-8"?>
<FolderSync xmlns="FolderHierarchy:">
    <SyncKey>0</SyncKey>
</FolderSync>
```

**Endpoint**:
```
POST /Microsoft-Server-ActiveSync?Cmd=FolderSync&User={username}&DeviceId={device_id}&DeviceType={device_type}
```

**Headers**:
- `Authorization: Basic {base64_credentials}`
- `Content-Type: application/vnd.ms-sync.wbxml`
- `MS-ASProtocolVersion: 14.1`

### Sync Command

**Purpose**: Synchronize items (emails) in a folder

**Request XML**:
```xml
<?xml version="1.0" encoding="utf-8"?>
<Sync xmlns="AirSync:">
    <Collections>
        <Collection>
            <SyncKey>0</SyncKey>
            <CollectionId>{folder_id}</CollectionId>
        </Collection>
    </Collections>
</Sync>
```

**Endpoint**:
```
POST /Microsoft-Server-ActiveSync?Cmd=Sync&User={username}&DeviceId={device_id}&DeviceType={device_type}
```

### OPTIONS Command

**Purpose**: Check server capabilities and authenticate

**Request**: Empty body

**Endpoint**:
```
OPTIONS /Microsoft-Server-ActiveSync
```

**Response Headers**:
- `MS-ASProtocolVersions`: Supported protocol versions (e.g., "14.0,14.1,16.0")
- `MS-ASProtocolCommands`: Supported commands (e.g., "Sync,SendMail,FolderSync")

## Error Handling

### HTTP Status Codes

- **200 OK**: Successful request
- **401 Unauthorized**: Invalid credentials
- **403 Forbidden**: Access denied
- **404 Not Found**: EAS endpoint not found
- **449**: Retry after provisioning (not yet implemented)
- **500**: Server error

### Error Conversion

```rust
match client.connect().await {
    Err(EasError::HttpError(e)) => {
        eprintln!("Network error: {}", e);
    }
    Err(EasError::AuthenticationFailed) => {
        eprintln!("Invalid username or password");
    }
    Err(EasError::ProtocolError(msg)) => {
        eprintln!("Protocol error: {}", msg);
    }
    Err(EasError::ConnectionError(msg)) => {
        eprintln!("Connection error: {}", msg);
    }
    Ok(()) => {
        println!("Connected!");
    }
}
```

## Protocol Versions

### EAS 14.1 (Default)

- Widely supported
- Basic email sync
- Folder sync
- Standard authentication

### EAS 16.0

- Enhanced features
- Better attachment handling
- Improved sync performance
- Not yet fully implemented

## Authentication

### HTTP Basic Authentication

MailArrow uses HTTP Basic Authentication:

1. Concatenate username and password with colon: `username:password`
2. Encode with Base64
3. Send in `Authorization` header: `Basic {base64_string}`

**Example**:
```
Username: user@example.com
Password: mypassword
Base64: dXNlckBleGFtcGxlLmNvbTpteXBhc3N3b3Jk
Header: Authorization: Basic dXNlckBleGFtcGxlLmNvbTpteXBhc3N3b3Jk
```

## Threading Model

### Thread Safety

EasClient is designed to be cloned and used across async tasks:

```rust
let client1 = client.clone();
let client2 = client.clone();

// Both clients share the same connection state
tokio::spawn(async move {
    client1.sync_folders().await
});

tokio::spawn(async move {
    client2.sync_emails("inbox").await
});
```

### Shared State

The following fields use `Arc<RwLock<T>>` for thread-safe sharing:
- `state: Arc<RwLock<ConnectionState>>`
- `policy_key: Arc<RwLock<Option<String>>>`

## Future Enhancements

### Planned Features

1. **XML Response Parsing**: Parse real server responses
2. **SendMail**: Send outgoing emails
3. **ItemOperations**: Retrieve specific items
4. **GetAttachment**: Download attachments
5. **MeetingResponse**: Handle meeting invitations
6. **Provision**: Handle device provisioning policies
7. **Search**: Search emails and folders

### Protocol Extensions

- Support for EAS 16.1
- Support for EAS 14.0 (backward compatibility)
- OAuth 2.0 authentication (for Office 365)

## References

- [MS-ASCMD: Exchange ActiveSync Command Reference Protocol](https://docs.microsoft.com/en-us/openspecs/exchange_server_protocols/ms-ascmd/)
- [MS-ASHTTP: Exchange ActiveSync HTTP Protocol](https://docs.microsoft.com/en-us/openspecs/exchange_server_protocols/ms-ashttp/)
- [peas - Python EAS Library](https://github.com/ReversecLabs/peas)
- [ActiveSync Documentation](https://github.com/alireza-es/ActiveSync)
