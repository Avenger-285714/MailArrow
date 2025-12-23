# Debugging Guide

## Authentication Failed Errors

When you see "Connection failed: Authentication failed", this guide will help you troubleshoot the issue.

## Testing with Local EAS Server

For local testing without a real Exchange server, use the included test server:

```bash
# Terminal 1: Start the test server
cd tools/selftests/eas
cargo run

# Terminal 2: Run MailArrow with debug logging
cd ../../..
RUST_LOG=debug cargo run
```

**Test credentials:**
- Server URL: `127.0.0.1:8080` (HTTP will be used automatically)
- Username: `test@example.com`
- Password: `password123`

**Note:** The URL normalization automatically uses HTTP for localhost/127.0.0.1 addresses for local testing.

## Step 1: Enable Debug Logging

Run the application with debug logging enabled:

```bash
RUST_LOG=debug cargo run
```

This will show detailed information about:
- The exact URL being contacted (HTTP vs HTTPS)
- HTTP request headers
- HTTP response status codes
- Response headers from the server

## Step 2: Check the Debug Output

Look for these key log messages:

### Connection Attempt
```
INFO mailarrow::eas::client: Starting EAS connection attempt
DEBUG mailarrow::eas::client: Building authentication header
DEBUG mailarrow::eas::client: Sending OPTIONS request to: http://127.0.0.1:8080/Microsoft-Server-ActiveSync
```
(Note: URL will be https:// for production servers, http:// for localhost)

### Successful Response
```
DEBUG mailarrow::eas::client: Received response with status: 200
INFO mailarrow::eas::client: Successfully connected to EAS server
```

### Failed Authentication (401)
```
DEBUG mailarrow::eas::client: Received response with status: 401
ERROR mailarrow::eas::client: HTTP 401 Unauthorized - Check username and password
```

### Failed Authentication (403)
```
DEBUG mailarrow::eas::client: Received response with status: 403
ERROR mailarrow::eas::client: HTTP 403 Forbidden - Access denied
```

### Endpoint Not Found (404)
```
DEBUG mailarrow::eas::client: Received response with status: 404
ERROR mailarrow::eas::client: HTTP 404 Not Found - EAS endpoint not found at: https://mail.example.com/Microsoft-Server-ActiveSync
```

## Step 3: Common Issues and Solutions

### Issue 1: Wrong Server URL

**Symptoms**:
- 404 Not Found error
- Connection timeout

**Solution**:
1. Verify the server URL with your email administrator
2. Common EAS endpoints:
   - `https://mail.example.com`
   - `https://exchange.example.com`
   - `https://outlook.office365.com` (for Office 365)

### Issue 2: Incorrect Credentials

**Symptoms**:
- 401 Unauthorized error

**Solution**:
1. Double-check username and password
2. For domain accounts, try format: `DOMAIN\username`
3. Check if account requires app-specific password
4. Verify account is not locked or disabled

### Issue 3: Permission Issues

**Symptoms**:
- 403 Forbidden error

**Solution**:
1. Check if EAS is enabled for your account
2. Verify mobile device policy allows EAS
3. Contact administrator to enable EAS access

### Issue 4: Certificate Issues (HTTPS vs HTTP)

**Symptoms**:
- SSL/TLS errors
- "certificate verify failed"
- "unexpected EOF" when connecting to local test server

**Solution**:
1. **For Production Servers**: Verify server uses valid SSL certificate
2. **For Local Testing**: 
   - Use explicit `http://` prefix: `http://127.0.0.1:8080`
   - Or the URL will auto-detect localhost/127.0.0.1 and use HTTP
   - Local IPs (127.0.0.1, localhost, 192.168.x.x, 10.x.x.x) default to HTTP
3. **For Self-signed Certificates** (testing only):
   - This is not currently supported
   - Future version will add option to trust self-signed certificates

### Issue 5: Firewall/Network Issues

**Symptoms**:
- Connection timeout
- "Network error"

**Solution**:
1. Check internet connection
2. Verify firewall allows HTTPS (port 443) or HTTP (port 80/8080)
3. Try from different network
4. Check if VPN is required

## Step 4: Test with curl

Verify the server responds correctly:

```bash
# Test OPTIONS request
curl -v -X OPTIONS \
  -H "Authorization: Basic $(echo -n 'username:password' | base64)" \
  https://mail.example.com/Microsoft-Server-ActiveSync

# Expected response: HTTP 200 with EAS headers
```

## Step 5: Check Server Capabilities

Look for these headers in the response (shown in debug log):

```
ms-asprotocolversions: 14.0,14.1,16.0
ms-asprotocolcommands: Sync,SendMail,FolderSync,...
```

If these headers are missing, the endpoint may not be a valid EAS server.

## Advanced Debugging

### Enable Trace Logging

For even more detailed logging:

```bash
RUST_LOG=trace cargo run
```

### Log Request/Response Bodies

Currently not implemented, but planned for future versions.

### Network Packet Capture

Use Wireshark or tcpdump to inspect network traffic:

```bash
# Capture HTTPS traffic (you'll need to decrypt it)
sudo tcpdump -i any -w capture.pcap port 443
```

## Reporting Issues

When reporting authentication issues, include:

1. Debug log output (redact sensitive information!)
2. Server type (Exchange Server version, Office 365, etc.)
3. HTTP status code received
4. Any relevant server headers
5. Whether curl test works

Example report:

```
Server: Exchange Server 2019
URL: https://mail.example.com
Status: 401 Unauthorized
Curl test: Works with same credentials
Log excerpt: [attach relevant lines]
```

## Security Notes

**NEVER** share:
- Your actual password
- Full authentication headers
- Sensitive log output

**DO** share:
- HTTP status codes
- Server headers (non-sensitive)
- URL format (with domain removed)
- Error messages
