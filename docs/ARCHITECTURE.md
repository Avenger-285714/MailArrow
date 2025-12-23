# MailArrow Architecture

## Overview

MailArrow is a lightweight email client built with Rust, utilizing the iced GUI framework and implementing the Exchange ActiveSync (EAS) protocol for email synchronization.

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────┐
│                    MailArrow Application                 │
├─────────────────────────────────────────────────────────┤
│                                                          │
│  ┌────────────────────────────────────────────────┐    │
│  │              UI Layer (iced)                    │    │
│  │  ┌──────────────┐      ┌──────────────────┐   │    │
│  │  │ LoginScreen  │      │   MainView       │   │    │
│  │  └──────────────┘      └──────────────────┘   │    │
│  └────────────────────────────────────────────────┘    │
│                        ▲                                │
│                        │                                │
│                        ▼                                │
│  ┌────────────────────────────────────────────────┐    │
│  │          Application State Manager              │    │
│  │    (Login State ↔ Main State Transitions)      │    │
│  └────────────────────────────────────────────────┘    │
│                        ▲                                │
│                        │                                │
│                        ▼                                │
│  ┌────────────────────────────────────────────────┐    │
│  │            EAS Protocol Layer                   │    │
│  │  ┌──────────────┐  ┌──────────────────────┐   │    │
│  │  │  EasClient   │  │  Protocol Generator   │   │    │
│  │  │  - Connect   │  │  - FolderSync XML    │   │    │
│  │  │  - Sync      │  │  - Sync XML          │   │    │
│  │  └──────────────┘  └──────────────────────┘   │    │
│  └────────────────────────────────────────────────┘    │
│                        ▲                                │
│                        │                                │
└────────────────────────┼────────────────────────────────┘
                         │
                         ▼
              ┌─────────────────────┐
              │   EAS Server        │
              │  (Exchange Server)  │
              └─────────────────────┘
```

## Module Structure

### 1. Main Application (`src/main.rs`)

**Purpose**: Application entry point and state management

**Key Components**:
- `MailArrow` struct: Main application state
- `AppState` enum: Tracks whether user is on login or main screen
- Message handling: Processes user interactions and async responses

### 2. EAS Protocol Module (`src/eas/`)

#### Key Structures:

```rust
pub struct EasClient {
    credentials: Credentials,
    client: Client,
    state: Arc<RwLock<ConnectionState>>,
    device_id: String,
    version: EasVersion,
}
```

### 3. UI Module (`src/ui/`)

See full documentation in individual module files.

## Debugging

### Enable Debug Logging

```bash
# Enable debug logs
RUST_LOG=debug cargo run

# Enable debug logs only for MailArrow
RUST_LOG=mailarrow=debug cargo run

# Enable trace logs for EAS client
RUST_LOG=mailarrow::eas::client=trace cargo run
```

### Common Authentication Issues

1. **401 Unauthorized**: Check username and password
2. **403 Forbidden**: Access denied - check account permissions
3. **404 Not Found**: EAS endpoint incorrect - verify server URL
4. **Network errors**: Check firewall and internet connection

See DEBUGGING.md for detailed troubleshooting guide.
