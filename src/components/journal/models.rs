use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Dream {
    pub id: String, // Keep as String for frontend to handle UUID
    pub text: String,
    pub interpretation: String,
    #[serde(rename = "created_at")]
    pub created_at: String, // ISO 8601 date string
    #[serde(default)]
    pub category: Option<String>, // Dream category
    #[serde(default)]
    pub tags: Vec<String>, // Optional tags
    #[serde(default)]
    pub is_favorite: bool, // Whether the dream is marked as favorite
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Pagination {
    pub current_page: usize,
    pub total_pages: usize,
    pub items_per_page: usize,
}

// Search filters for dreams
#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct DreamSearchFilters {
    pub search_query: String,
    pub date_filter: String,
    pub category: String,
    pub tags: Vec<String>,
    pub favorites_only: bool,
}
