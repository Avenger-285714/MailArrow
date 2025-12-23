/// Unit tests for EAS types and validation
/// 
/// These tests verify the behavior of EAS data structures,
/// especially the URL normalization functionality.

#[cfg(test)]
mod tests {
    use mailarrow::eas::types::Credentials;

    /// Test URL normalization with various input formats
    /// 
    /// Verifies that URLs without a scheme get https:// prepended,
    /// while URLs with an existing scheme remain unchanged.
    #[test]
    fn test_url_normalization() {
        // Test case 1: URL without scheme should get https:// added
        let result = Credentials::normalize_url("mail.example.com");
        assert_eq!(result.unwrap(), "https://mail.example.com");

        // Test case 2: URL with https:// should remain unchanged
        let result = Credentials::normalize_url("https://mail.example.com");
        assert_eq!(result.unwrap(), "https://mail.example.com");

        // Test case 3: URL with http:// should remain unchanged
        let result = Credentials::normalize_url("http://mail.example.com");
        assert_eq!(result.unwrap(), "http://mail.example.com");
    }

    /// Test URL normalization with whitespace
    /// 
    /// Verifies that leading and trailing whitespace is properly trimmed
    /// before URL normalization.
    #[test]
    fn test_url_normalization_with_whitespace() {
        // Leading and trailing whitespace should be trimmed
        let result = Credentials::normalize_url("  mail.example.com  ");
        assert_eq!(result.unwrap(), "https://mail.example.com");

        // Whitespace with scheme
        let result = Credentials::normalize_url("  https://mail.example.com  ");
        assert_eq!(result.unwrap(), "https://mail.example.com");
    }

    /// Test URL normalization error cases
    /// 
    /// Verifies that empty or whitespace-only URLs are rejected
    /// with appropriate error messages.
    #[test]
    fn test_url_normalization_errors() {
        // Empty string should return error
        let result = Credentials::normalize_url("");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Server URL cannot be empty");

        // Only whitespace should return error
        let result = Credentials::normalize_url("   ");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Server URL cannot be empty");
    }

    /// Test URL normalization with ports
    /// 
    /// Verifies that port numbers are preserved during normalization.
    #[test]
    fn test_url_normalization_with_ports() {
        // URL with port should work
        let result = Credentials::normalize_url("mail.example.com:8080");
        assert_eq!(result.unwrap(), "https://mail.example.com:8080");

        // URL with scheme and port
        let result = Credentials::normalize_url("https://mail.example.com:8080");
        assert_eq!(result.unwrap(), "https://mail.example.com:8080");
    }

    /// Test URL normalization with IP addresses
    /// 
    /// Verifies that IP addresses (IPv4 and localhost) are handled correctly.
    #[test]
    fn test_url_normalization_with_ip() {
        // IPv4 address
        let result = Credentials::normalize_url("192.168.1.100");
        assert_eq!(result.unwrap(), "https://192.168.1.100");

        // IPv4 with port
        let result = Credentials::normalize_url("192.168.1.100:8080");
        assert_eq!(result.unwrap(), "https://192.168.1.100:8080");

        // localhost
        let result = Credentials::normalize_url("localhost");
        assert_eq!(result.unwrap(), "https://localhost");
    }
}

