use serde_json::Value;
use substring::Substring;

/// Attempt to extract the first valid JSON object found within a string. The
/// JSON object must start with a `{` and end with a matching `}`.
/// Returns `Some(Value)` if successful or `None` if no valid JSON could be
/// parsed.
pub fn try_extract(body: &str) -> Option<Value> {
    let (Some(start), Some(end)) = (body.find('{'), body.rfind('}')) else {
        return None;
    };

    let data = body.substring(start, end + 1);

    serde_json::from_str::<Value>(data).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_extract_valid_json() {
        let input = "Some text {\"command\": \"ls -la\"} some more text";
        let result = try_extract(input);
        assert!(result.is_some());
        let json_val = result.unwrap();
        assert_eq!(json_val["command"], "ls -la");
    }

    #[test]
    fn test_try_extract_no_json() {
        let input = "There is no JSON here";
        assert!(try_extract(input).is_none());
    }
    
    #[test]
    fn test_try_extract_invalid_json() {
        let input = "Some text {\"command\": \"ls -la some more text";
        assert!(try_extract(input).is_none());
    }
} 