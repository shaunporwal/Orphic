// Import the orphic crate for testing

// Import all public API items
extern crate orphic;

#[tokio::test]
async fn test_prompt_integration() {
    let prompt = orphic::prompts::get_prompt("assistant_user");
    assert!(!prompt.is_empty());
}

#[tokio::test]
async fn test_json_extraction() {
    let json_str = "Test message {\"command\": \"echo hello\"}";
    let extracted = orphic::utils::try_extract(json_str);
    assert!(extracted.is_some());
    
    let value = extracted.unwrap();
    assert_eq!(value["command"], "echo hello");
}

// Note: These tests don't actually hit the OpenAI API
// Integration tests with actual API calls would require API keys
// and would incur costs, so we're keeping them simple 