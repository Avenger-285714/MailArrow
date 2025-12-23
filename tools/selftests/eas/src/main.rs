/// EAS Test Server
/// 
/// A simple local EAS (Exchange ActiveSync) server for testing the MailArrow client.
/// This server implements the minimum EAS protocol needed to test client functionality.
/// 
/// # Features
/// 
/// - HTTP Basic Authentication
/// - OPTIONS endpoint for capability discovery
/// - FolderSync command with mock folder data
/// - Sync command with mock email data
/// 
/// # Usage
/// 
/// ```bash
/// cargo run
/// ```
/// 
/// Server listens on http://127.0.0.1:8080 by default.
/// 
/// Test credentials:
/// - Username: test@example.com
/// - Password: password123

use axum::{
    Router,
    routing::{options, post},
    extract::Query,
    http::{StatusCode, HeaderMap, header},
    response::{IntoResponse, Response},
};
use base64::{Engine, engine::general_purpose::STANDARD as BASE64};
use serde::Deserialize;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing::{info, debug, warn};

/// Query parameters for EAS requests
#[derive(Debug, Deserialize)]
struct EasQuery {
    #[serde(rename = "Cmd")]
    cmd: Option<String>,
    #[serde(rename = "User")]
    user: Option<String>,
    #[serde(rename = "DeviceId")]
    device_id: Option<String>,
    #[serde(rename = "DeviceType")]
    device_type: Option<String>,
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_target(true)
        .with_line_number(true)
        .init();

    info!("Starting EAS Test Server");

    // Build router
    let app = Router::new()
        .route("/Microsoft-Server-ActiveSync", options(handle_options))
        .route("/Microsoft-Server-ActiveSync", post(handle_post))
        .layer(TraceLayer::new_for_http());

    // Start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    info!("Listening on http://{}", addr);
    info!("Test credentials: test@example.com / password123");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// Handle OPTIONS request - returns EAS capabilities
/// 
/// # Returns
/// 
/// Response with EAS protocol versions and commands in headers
async fn handle_options(headers: HeaderMap) -> Response {
    debug!("Received OPTIONS request");
    
    // Check authorization
    if !check_auth(&headers) {
        warn!("OPTIONS: Authentication failed");
        return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
    }
    
    info!("OPTIONS: Authentication successful");
    
    // Return EAS capabilities
    let mut response_headers = HeaderMap::new();
    response_headers.insert(
        "MS-ASProtocolVersions",
        "14.0,14.1,16.0".parse().unwrap()
    );
    response_headers.insert(
        "MS-ASProtocolCommands",
        "Sync,SendMail,FolderSync,FolderCreate,FolderDelete,FolderUpdate,GetItemEstimate,ItemOperations".parse().unwrap()
    );
    
    (StatusCode::OK, response_headers, "").into_response()
}

/// Handle POST request - process EAS commands
/// 
/// # Arguments
/// 
/// * `query` - Query parameters with command and user info
/// * `headers` - Request headers including authorization
/// * `body` - Request body with XML data
/// 
/// # Returns
/// 
/// XML response based on the command type
async fn handle_post(
    Query(query): Query<EasQuery>,
    headers: HeaderMap,
    body: String,
) -> Response {
    debug!("Received POST request: {:?}", query);
    
    // Check authorization
    if !check_auth(&headers) {
        warn!("POST: Authentication failed");
        return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
    }
    
    let cmd = query.cmd.as_deref().unwrap_or("");
    info!("Processing command: {}", cmd);
    
    match cmd {
        "FolderSync" => handle_folder_sync(body).await,
        "Sync" => handle_sync(body).await,
        _ => {
            warn!("Unknown command: {}", cmd);
            (StatusCode::BAD_REQUEST, format!("Unknown command: {}", cmd)).into_response()
        }
    }
}

/// Check HTTP Basic Authentication
/// 
/// Validates credentials against test user (test@example.com / password123)
/// 
/// # Arguments
/// 
/// * `headers` - Request headers containing Authorization header
/// 
/// # Returns
/// 
/// true if authentication succeeds, false otherwise
fn check_auth(headers: &HeaderMap) -> bool {
    let auth_header = match headers.get(header::AUTHORIZATION) {
        Some(h) => h,
        None => {
            debug!("No Authorization header");
            return false;
        }
    };
    
    let auth_str = match auth_header.to_str() {
        Ok(s) => s,
        Err(_) => {
            debug!("Invalid Authorization header format");
            return false;
        }
    };
    
    if !auth_str.starts_with("Basic ") {
        debug!("Not Basic authentication");
        return false;
    }
    
    let encoded = &auth_str[6..];
    let decoded = match BASE64.decode(encoded) {
        Ok(d) => d,
        Err(_) => {
            debug!("Failed to decode Base64");
            return false;
        }
    };
    
    let credentials = match String::from_utf8(decoded) {
        Ok(s) => s,
        Err(_) => {
            debug!("Invalid UTF-8 in credentials");
            return false;
        }
    };
    
    debug!("Checking credentials: {}", credentials);
    
    // Test credentials: test@example.com:password123
    credentials == "test@example.com:password123"
}

/// Handle FolderSync command
/// 
/// Returns mock folder hierarchy including Inbox, Sent Items, and Drafts
/// 
/// # Arguments
/// 
/// * `_body` - Request body (currently unused)
/// 
/// # Returns
/// 
/// XML response with folder list
async fn handle_folder_sync(_body: String) -> Response {
    info!("Handling FolderSync command");
    
    // Return mock folder structure
    let xml = r#"<?xml version="1.0" encoding="utf-8"?>
<FolderSync xmlns="FolderHierarchy:">
    <Status>1</Status>
    <SyncKey>1</SyncKey>
    <Changes>
        <Count>3</Count>
        <Add>
            <ServerId>1</ServerId>
            <ParentId>0</ParentId>
            <DisplayName>Inbox</DisplayName>
            <Type>2</Type>
        </Add>
        <Add>
            <ServerId>2</ServerId>
            <ParentId>0</ParentId>
            <DisplayName>Sent Items</DisplayName>
            <Type>5</Type>
        </Add>
        <Add>
            <ServerId>3</ServerId>
            <ParentId>0</ParentId>
            <DisplayName>Drafts</DisplayName>
            <Type>3</Type>
        </Add>
    </Changes>
</FolderSync>"#;
    
    let mut response_headers = HeaderMap::new();
    response_headers.insert(
        header::CONTENT_TYPE,
        "application/vnd.ms-sync.wbxml".parse().unwrap()
    );
    
    (StatusCode::OK, response_headers, xml).into_response()
}

/// Handle Sync command
/// 
/// Returns mock email data for the requested folder
/// 
/// # Arguments
/// 
/// * `_body` - Request body (currently unused)
/// 
/// # Returns
/// 
/// XML response with email list
async fn handle_sync(_body: String) -> Response {
    info!("Handling Sync command");
    
    // Return mock email data
    let xml = r#"<?xml version="1.0" encoding="utf-8"?>
<Sync xmlns="AirSync:">
    <Collections>
        <Collection>
            <SyncKey>1</SyncKey>
            <CollectionId>1</CollectionId>
            <Status>1</Status>
            <Commands>
                <Add>
                    <ServerId>msg1</ServerId>
                    <ApplicationData>
                        <Subject xmlns="Email:">Welcome to MailArrow</Subject>
                        <From xmlns="Email:">admin@example.com</From>
                        <DateReceived xmlns="Email:">2024-01-01T10:00:00Z</DateReceived>
                        <Body xmlns="AirSyncBase:">Welcome to MailArrow testing!</Body>
                    </ApplicationData>
                </Add>
                <Add>
                    <ServerId>msg2</ServerId>
                    <ApplicationData>
                        <Subject xmlns="Email:">Test Email</Subject>
                        <From xmlns="Email:">test@example.com</From>
                        <DateReceived xmlns="Email:">2024-01-01T11:00:00Z</DateReceived>
                        <Body xmlns="AirSyncBase:">This is a test email.</Body>
                    </ApplicationData>
                </Add>
            </Commands>
        </Collection>
    </Collections>
</Sync>"#;
    
    let mut response_headers = HeaderMap::new();
    response_headers.insert(
        header::CONTENT_TYPE,
        "application/vnd.ms-sync.wbxml".parse().unwrap()
    );
    
    (StatusCode::OK, response_headers, xml).into_response()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test authentication validation with correct credentials
    #[test]
    fn test_check_auth_valid() {
        let mut headers = HeaderMap::new();
        // test@example.com:password123 in Base64
        let auth = format!("Basic {}", BASE64.encode("test@example.com:password123"));
        headers.insert(header::AUTHORIZATION, auth.parse().unwrap());
        
        assert!(check_auth(&headers), "Valid credentials should authenticate");
    }

    /// Test authentication validation with invalid credentials
    #[test]
    fn test_check_auth_invalid() {
        let mut headers = HeaderMap::new();
        // wrong credentials
        let auth = format!("Basic {}", BASE64.encode("wrong:credentials"));
        headers.insert(header::AUTHORIZATION, auth.parse().unwrap());
        
        assert!(!check_auth(&headers), "Invalid credentials should fail");
    }

    /// Test authentication with missing header
    #[test]
    fn test_check_auth_missing() {
        let headers = HeaderMap::new();
        assert!(!check_auth(&headers), "Missing auth header should fail");
    }
}
