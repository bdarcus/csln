use schemars::{JsonSchema};
use serde::{Serialize, Deserialize};
//use edtf::DateComplete;

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
pub struct InputReference {
    pub id: Option<String>,
    pub title: Option<String>,
    pub author: Option<Vec<String>>,
    pub editor: Option<Vec<String>>,
    pub translator: Option<Vec<String>>,
    pub issued: Option<String>,
    pub publisher: Option<Vec<String>>,
    pub url: Option<String>,
    pub accessed: Option<String>,
    pub note: Option<String>,
}