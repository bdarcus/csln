use schemars::{JsonSchema};
use serde::{Serialize, Deserialize};
//use edtf::DateComplete;

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
pub struct InputReference {
    pub id: String,
    pub title: String,
    pub author: Option<Vec<String>>,
    pub editor: Option<Vec<String>>,
    pub issued: String,
    pub publisher: Option<Vec<String>>,
    pub url: String,
    pub accessed: String,
    pub note: String
}