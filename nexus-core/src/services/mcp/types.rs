use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
pub enum TextOperation {
    Count,    // Count characters, words, etc.
    Validate, // Check if text matches certain criteria
    Format,   // Format or clean text
    Analyze,  // Analyze sentiment or content
}

impl TextOperation {
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "count" => Ok(Self::Count),
            "validate" => Ok(Self::Validate),
            "format" => Ok(Self::Format),
            "analyze" => Ok(Self::Analyze),
            _ => Err(format!(
                "Unknown operation: {}. Supported operations: count, validate, format, analyze",
                s
            )),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolMeaning {
    pub symbol: String,
    pub meaning: String,
    pub contexts: Vec<String>,
}
