use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct Message {
    pub role: String,
    pub content: String,
}

impl Message {
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: "user".to_string(),
            content: content.into(),
        }
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: "assistant".to_string(),
            content: content.into(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SymbolMeaning {
    pub symbol: String,
    pub contexts: Vec<String>,
    pub meaning: String,
}

#[derive(Clone, Debug)]
pub struct ModelConfig {
    pub name: String,
    pub temperature: f32,
    pub top_p: f32,
    pub max_tokens: Option<u32>,
}

impl Default for ModelConfig {
    fn default() -> Self {
        Self {
            name: "openai/gpt-4.1".to_string(),
            temperature: 0.7,
            top_p: 0.9,
            max_tokens: None,
        }
    }
}
