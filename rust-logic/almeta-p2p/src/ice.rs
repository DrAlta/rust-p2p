
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ICE {
    #[serde(rename = "Media")]
    pub media: String,
    #[serde(rename = "Index")]
    pub index: i64, 
    #[serde(rename = "Name")]
    pub name: String,
}
impl ICE {
    pub fn new(media: String, index: i64, name: String) -> Self {
        Self{media, index, name}
    }
}