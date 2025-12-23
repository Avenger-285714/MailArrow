# MailArrow

A fast and lightweight email client built with Rust and the iced GUI framework, supporting Exchange ActiveSync (EAS) protocol.

## Features

- 🚀 **Fast and Lightweight**: Built with Rust for optimal performance
- 🎨 **Modern GUI**: Clean user interface powered by iced framework
- 📧 **EAS Protocol Support**: Connect to Exchange ActiveSync servers
- 🔒 **Secure**: Basic authentication with secure credential handling
- 📱 **Cross-platform**: Runs on Linux, macOS, and Windows

## Current Implementation

### Supported Features

- ✅ EAS protocol connection and authentication
- ✅ Folder synchronization (Inbox, Sent Items, Drafts, etc.)
- ✅ Email listing and viewing
- ✅ Three-panel interface (Folders, Email List, Email Content)
- ✅ Login screen with server configuration

### Architecture

The project is organized into the following modules:

```
src/
├── eas/              # EAS protocol implementation
│   ├── client.rs     # EAS client with connection management
│   ├── protocol.rs   # EAS protocol XML generation
│   ├── types.rs      # Data structures (Email, Folder, etc.)
│   └── mod.rs        # Module exports
├── ui/               # User interface components
│   ├── login.rs      # Login screen
│   ├── main_view.rs  # Main email viewing interface
│   └── mod.rs        # Module exports
└── main.rs           # Application entry point
```

## Building and Running

### Prerequisites

- Rust 1.70 or later
- Cargo (comes with Rust)

### Build

```bash
cargo build --release
```

### Run

```bash
cargo run
```

Or run the release build:

```bash
./target/release/mailarrow
```

## Usage

1. **Launch the application**: Run `cargo run` or execute the built binary
2. **Enter credentials**:
   - **Server URL**: Enter your EAS server URL
     - Can be in the format: `mail.example.com` (https:// will be added automatically)
     - Or with explicit protocol: `https://mail.example.com`
   - **Username**: Your email username
   - **Password**: Your email password
   - **Domain** (optional): Your domain if required
3. **Connect**: Click the "Connect" button
4. **Navigate**: 
   - Select folders from the left panel
   - View email list in the middle panel
   - Read email content in the right panel

### Input Validation

The application performs the following validations on connection:
- Server URL cannot be empty
- Username cannot be empty
- Password cannot be empty
- Server URL is automatically normalized (adds https:// if no protocol is specified)

## EAS Protocol Implementation

This project implements Exchange ActiveSync (EAS) protocol for email synchronization. The implementation is inspired by:

- [peas](https://github.com/ReversecLabs/peas) - Python EAS client
- [ActiveSync](https://github.com/alireza-es/ActiveSync) - ActiveSync protocol implementation
- [grommunio-sync](https://github.com/grommunio/grommunio-sync) - Open-source EAS server

### Supported EAS Commands

- **FolderSync**: Synchronize folder hierarchy
- **Sync**: Synchronize emails from specific folders
- **OPTIONS**: Check server capabilities

### EAS Versions Supported

- EAS 14.1 (default)
- EAS 16.0 (configurable)

## Development

### Project Structure

- **eas/client.rs**: Handles HTTP communication with EAS servers
- **eas/protocol.rs**: Generates EAS XML requests
- **eas/types.rs**: Data models for emails, folders, and credentials
- **ui/login.rs**: Login screen UI component
- **ui/main_view.rs**: Main email viewing interface
- **main.rs**: Application state management and message handling

### Dependencies

Key dependencies:
- `iced`: GUI framework
- `tokio`: Async runtime
- `reqwest`: HTTP client
- `quick-xml`: XML parsing and generation
- `serde`: Serialization/deserialization
- `base64`: Base64 encoding for authentication

## Roadmap

Future improvements planned:
- [ ] Full XML response parsing for EAS protocol
- [ ] Email composition and sending
- [ ] Attachment handling
- [ ] Contact and calendar sync
- [ ] Search functionality
- [ ] Multi-account support
- [ ] Offline mode with local caching
- [ ] Email threading
- [ ] Rich text email rendering

## License

This project is licensed under the GNU Affero General Public License v3.0 (AGPL-3.0).

See [LICENSE](LICENSE) for details.

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## Notes

- This is an initial implementation focusing on basic EAS functionality
- Currently uses mock data for demonstration purposes (folders and emails)
- Full EAS protocol parsing will be implemented in future updates
- The application currently does not support IMAP, SMTP, or POP3 protocols as per the initial requirements
- Connection state is managed with Arc<RwLock> for thread-safe shared state

## Known Limitations

- XML response parsing is currently stubbed out with placeholder functions
- FolderSync and Sync commands return mock data instead of parsing real responses
- Some code duplication in connection handling (will be optimized in future)
- Limited error handling and recovery
- No offline support or caching

