/// Unit tests for EAS protocol XML generation
/// 
/// These tests verify that the EAS protocol module correctly
/// generates XML requests for various EAS commands like FolderSync and Sync.

#[cfg(test)]
mod tests {
    use mailarrow::eas::protocol::{create_folder_sync_request, create_sync_request};

    /// Test FolderSync XML generation with initial sync key
    /// 
    /// Verifies that the FolderSync command generates well-formed XML
    /// with the correct structure and elements.
    #[test]
    fn test_folder_sync_request_generation() {
        // Initial sync with sync_key "0"
        let xml = create_folder_sync_request("0").expect("Should generate XML");
        
        // Verify XML contains required elements
        assert!(xml.contains("<?xml"), "Should have XML declaration");
        assert!(xml.contains("<FolderSync"), "Should have FolderSync element");
        assert!(xml.contains("<SyncKey>0</SyncKey>"), "Should have SyncKey element");
        assert!(xml.contains("</FolderSync>"), "Should close FolderSync element");
    }

    /// Test FolderSync with non-zero sync key
    /// 
    /// Verifies that subsequent syncs with existing sync keys
    /// embed the key correctly in the XML.
    #[test]
    fn test_folder_sync_with_sync_key() {
        // Sync with existing sync_key
        let xml = create_folder_sync_request("12345").expect("Should generate XML");
        
        // Verify sync key is correctly embedded
        assert!(xml.contains("<SyncKey>12345</SyncKey>"), 
                "Should contain the provided sync key");
    }

    /// Test Sync command XML generation
    /// 
    /// Verifies that the Sync command generates well-formed XML
    /// with collection information.
    #[test]
    fn test_sync_request_generation() {
        // Initial sync with sync_key "0" for inbox folder
        let xml = create_sync_request("0", "inbox_123").expect("Should generate XML");
        
        // Verify XML contains required elements
        assert!(xml.contains("<?xml"), "Should have XML declaration");
        assert!(xml.contains("<Sync"), "Should have Sync element");
        assert!(xml.contains("<Collections>"), "Should have Collections element");
        assert!(xml.contains("<Collection>"), "Should have Collection element");
        assert!(xml.contains("<SyncKey>0</SyncKey>"), "Should have SyncKey");
        assert!(xml.contains("<CollectionId>inbox_123</CollectionId>"), 
                "Should have CollectionId");
        assert!(xml.contains("</Collection>"), "Should close Collection");
        assert!(xml.contains("</Collections>"), "Should close Collections");
        assert!(xml.contains("</Sync>"), "Should close Sync");
    }

    /// Test Sync with different folder IDs
    /// 
    /// Verifies that the Sync command correctly handles different
    /// folder identifiers.
    #[test]
    fn test_sync_request_with_different_folders() {
        // Sent items folder
        let xml = create_sync_request("0", "sent_456").expect("Should generate XML");
        assert!(xml.contains("<CollectionId>sent_456</CollectionId>"),
                "Should contain sent folder ID");

        // Drafts folder
        let xml = create_sync_request("0", "drafts_789").expect("Should generate XML");
        assert!(xml.contains("<CollectionId>drafts_789</CollectionId>"),
                "Should contain drafts folder ID");
    }

    /// Test XML well-formedness
    /// 
    /// Verifies that generated XML has balanced tags.
    #[test]
    fn test_xml_well_formed() {
        // FolderSync XML should be well-formed
        let xml = create_folder_sync_request("0").expect("Should generate XML");
        
        // Basic check: count angle brackets
        let open_brackets = xml.matches('<').count();
        let close_brackets = xml.matches('>').count();
        assert_eq!(open_brackets, close_brackets, 
                   "XML should have matching angle brackets");

        // Sync XML should be well-formed
        let xml = create_sync_request("0", "test").expect("Should generate XML");
        let open_brackets = xml.matches('<').count();
        let close_brackets = xml.matches('>').count();
        assert_eq!(open_brackets, close_brackets,
                   "XML should have matching angle brackets");
    }
}

