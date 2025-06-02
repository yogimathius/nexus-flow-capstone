use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Dream {
    pub id: String,
    pub text: String,
    pub interpretation: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub is_favorite: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Pagination {
    pub current_page: usize,
    pub total_pages: usize,
    pub items_per_page: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct DreamSearchFilters {
    pub search_query: String,
    pub date_filter: String,
    pub category: String,
    pub tags: Vec<String>,
    pub favorites_only: bool,
}
