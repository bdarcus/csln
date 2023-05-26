use schemars::{JsonSchema};
use serde::{Serialize, Deserialize};
//use edtf::DateComplete;

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct InputReference {
    pub id: String,
    pub title: String,
    pub authors: Option<Vec<String>>,
    pub editors: Option<Vec<String>>,
    pub issued: String,
    pub publisher: Option<Vec<String>>,
    pub url: String,
    pub accessed: String,
    pub note: String
}