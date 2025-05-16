use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DreamRequest {
    pub dream_text: String,
    pub save: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DreamResponse {
    pub interpretation: String,
    pub status: String,
    pub id: Option<String>,
} 