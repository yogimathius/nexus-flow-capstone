use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Dream {
    pub id: String,
    pub text: String,
    pub interpretation: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
}
