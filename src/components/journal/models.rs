use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Dream {
    pub id: String, // Keep as String for frontend to handle UUID
    pub text: String,
    pub interpretation: String,
    #[serde(rename = "created_at")]
    pub created_at: String, // ISO 8601 date string
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Pagination {
    pub current_page: usize,
    pub total_pages: usize,
    pub items_per_page: usize,
}
