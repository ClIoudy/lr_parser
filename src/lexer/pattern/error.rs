#[derive(PartialEq, Eq, Hash)]
pub struct PatternError {
    message: String,
}

impl PatternError {
    pub fn pattern_matches_empty_string(pattern: &str) -> Self {
        Self {
            message: format!("pattern {} cannot match empty string", pattern),
        }
    }
}

impl std::fmt::Debug for PatternError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::fmt::Display for PatternError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for PatternError {}

impl From<regex::Error> for PatternError {
    fn from(value: regex::Error) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_error_empty_string_format() {
        let error = PatternError::pattern_matches_empty_string("[0-9]*");
        let error_msg = format!("{:?}", error);

        assert!(error_msg.contains("pattern"), "Error message should contain 'pattern'");
        assert!(error_msg.contains("[0-9]*"), "Error message should contain the pattern");
        assert!(error_msg.contains("cannot match empty string"), "Error message should mention empty string");
    }

    #[test]
    fn test_pattern_error_from_regex_error() {
        let regex_err = regex::Regex::new("[invalid(regex").unwrap_err();
        let pattern_err: PatternError = regex_err.into();
        let error_msg = format!("{:?}", pattern_err);

        assert!(!error_msg.is_empty(), "Error message should not be empty");
    }

    #[test]
    fn test_pattern_error_display() {
        let error = PatternError::pattern_matches_empty_string("a*");
        let debug_msg = format!("{:?}", error);
        let display_msg = format!("{}", error);

        assert_eq!(debug_msg, display_msg, "Display and Debug should produce same output");
    }

    #[test]
    fn test_pattern_error_is_error() {
        let error = PatternError::pattern_matches_empty_string(".*");
        
        // Verify it implements Error trait
        let _: &dyn std::error::Error = &error;
    }

    #[test]
    fn test_pattern_error_equality() {
        let error1 = PatternError::pattern_matches_empty_string("a*");
        let error2 = PatternError::pattern_matches_empty_string("a*");
        let error3 = PatternError::pattern_matches_empty_string("b*");

        assert_eq!(error1, error2, "Same pattern should produce equal errors");
        assert_ne!(error1, error3, "Different patterns should produce different errors");
    }
}