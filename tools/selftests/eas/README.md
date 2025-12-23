# EAS Test Server

A lightweight local EAS (Exchange ActiveSync) server for testing the MailArrow email client.

## Purpose

This test server implements the minimum EAS protocol functionality required to test the MailArrow client locally without needing access to a real Exchange server.

## Features

- **HTTP Basic Authentication**: Validates credentials
- **OPTIONS Endpoint**: Returns EAS capabilities
- **FolderSync Command**: Returns mock folder hierarchy
- **Sync Command**: Returns mock email data
- **Detailed Logging**: Debug connection and authentication issues

## Running the Server

```bash
cd tools/selftests/eas
cargo run
```

The server will start on `http://127.0.0.1:8080`.

## Test Credentials

Use these credentials in the MailArrow client to connect to the test server:

- **Server URL**: `http://127.0.0.1:8080` or `127.0.0.1:8080`
- **Username**: `test@example.com`
- **Password**: `password123`
- **Domain**: (leave empty)

## Testing with MailArrow

1. Start the test server:
   ```bash
   cd tools/selftests/eas
   cargo run
   ```

2. In another terminal, run MailArrow:
   ```bash
   cd ../../../
   RUST_LOG=debug cargo run
   ```

3. In the MailArrow login screen, enter:
   - Server URL: `127.0.0.1:8080`
   - Username: `test@example.com`
   - Password: `password123`

4. Click "Connect" - you should see:
   - Connection successful
   - Three folders: Inbox, Sent Items, Drafts
   - Two mock emails in the selected folder

## Mock Data

### Folders

The server returns three folders:

1. **Inbox** (ID: 1, Type: 2)
2. **Sent Items** (ID: 2, Type: 5)
3. **Drafts** (ID: 3, Type: 3)

### Emails

The server returns two mock emails:

1. **Welcome to MailArrow**
   - From: admin@example.com
   - Date: 2024-01-01T10:00:00Z
   - Body: Welcome to MailArrow testing!

2. **Test Email**
   - From: test@example.com
   - Date: 2024-01-01T11:00:00Z
   - Body: This is a test email.

## Logging

The server uses tracing for detailed logging. All requests are logged including:

- Authentication attempts
- Command types (OPTIONS, FolderSync, Sync)
- Success/failure status

## Testing with curl

You can also test the server with curl:

### Test OPTIONS (Capabilities)

```bash
curl -v -X OPTIONS \
  -H "Authorization: Basic $(echo -n 'test@example.com:password123' | base64)" \
  http://127.0.0.1:8080/Microsoft-Server-ActiveSync
```

Expected: HTTP 200 with MS-ASProtocolVersions header

### Test FolderSync

```bash
curl -v -X POST \
  -H "Authorization: Basic $(echo -n 'test@example.com:password123' | base64)" \
  -H "Content-Type: application/vnd.ms-sync.wbxml" \
  "http://127.0.0.1:8080/Microsoft-Server-ActiveSync?Cmd=FolderSync&User=test@example.com&DeviceId=TEST001&DeviceType=MailArrow" \
  -d '<?xml version="1.0"?><FolderSync xmlns="FolderHierarchy:"><SyncKey>0</SyncKey></FolderSync>'
```

Expected: HTTP 200 with folder list XML

### Test Authentication Failure

```bash
curl -v -X OPTIONS \
  -H "Authorization: Basic $(echo -n 'wrong:password' | base64)" \
  http://127.0.0.1:8080/Microsoft-Server-ActiveSync
```

Expected: HTTP 401 Unauthorized

## Architecture

```
┌─────────────────────────────────────────┐
│         MailArrow Client                │
│      (http://localhost:?????)           │
└──────────────┬──────────────────────────┘
               │
               │ HTTP Requests
               │ (Basic Auth)
               ▼
┌──────────────────────────────────────────┐
│      EAS Test Server                     │
│      (http://127.0.0.1:8080)            │
├──────────────────────────────────────────┤
│  ┌────────────────────────────────────┐ │
│  │  Authentication Layer              │ │
│  │  - Check Basic Auth header         │ │
│  │  - Validate credentials            │ │
│  └────────────────────────────────────┘ │
│  ┌────────────────────────────────────┐ │
│  │  Command Router                    │ │
│  │  - OPTIONS → Capabilities          │ │
│  │  - FolderSync → Mock Folders       │ │
│  │  - Sync → Mock Emails              │ │
│  └────────────────────────────────────┘ │
└──────────────────────────────────────────┘
```

## Implementation Details

### Authentication

The server validates HTTP Basic Authentication:

1. Extracts Authorization header
2. Decodes Base64 credentials
3. Compares against test credentials
4. Returns 401 if invalid

### Command Handling

**OPTIONS**: Returns supported protocol versions and commands in headers

**FolderSync**: Returns XML with mock folder hierarchy:
- Uses hardcoded SyncKey
- Returns 3 folders
- Includes folder IDs, names, and types

**Sync**: Returns XML with mock email list:
- Uses hardcoded SyncKey
- Returns 2 emails
- Includes basic email fields

## Running Tests

The server includes unit tests for authentication:

```bash
cargo test
```

Tests verify:
- Valid credentials authenticate successfully
- Invalid credentials are rejected
- Missing auth header is rejected

## Limitations

This is a minimal test server with the following limitations:

1. **No State**: Server doesn't maintain any state between requests
2. **No Real Sync**: Always returns same mock data
3. **No XML Parsing**: Doesn't parse request bodies
4. **Limited Commands**: Only OPTIONS, FolderSync, and Sync
5. **Single User**: Only one test account
6. **No WBXML**: Uses plain XML instead of WBXML encoding
7. **No TLS**: HTTP only (not HTTPS)

## Future Enhancements

Potential improvements:

- [ ] Parse XML request bodies
- [ ] Support WBXML encoding
- [ ] Add more EAS commands (SendMail, ItemOperations)
- [ ] Support multiple test accounts
- [ ] Add TLS support
- [ ] Implement stateful sync keys
- [ ] Add more mock data options
- [ ] Configuration file for test data

## Troubleshooting

### Server won't start

- Check port 8080 is not already in use
- Try a different port by modifying `main.rs`

### Client can't connect

- Verify server is running (`cargo run`)
- Check firewall settings
- Ensure URL is `http://127.0.0.1:8080` (not https)

### Authentication fails

- Verify credentials exactly: `test@example.com` / `password123`
- Check Base64 encoding is correct
- Look at server logs for details

### No folders/emails shown

- Check server logs for command processing
- Verify FolderSync and Sync commands are reaching server
- Check client is parsing responses correctly

## Development

### Adding New Commands

To add a new EAS command:

1. Add command handler function in `main.rs`
2. Add case in `handle_post` match statement
3. Create mock response XML
4. Add tests for the new command

### Modifying Mock Data

To change the mock folders or emails:

1. Edit `handle_folder_sync` for folders
2. Edit `handle_sync` for emails
3. Follow EAS XML schema format

## References

- [MS-ASCMD: Exchange ActiveSync Command Protocol](https://docs.microsoft.com/en-us/openspecs/exchange_server_protocols/ms-ascmd/)
- [Axum Web Framework](https://docs.rs/axum/)
- [MailArrow Documentation](../../../docs/)
