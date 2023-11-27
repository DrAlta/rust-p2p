
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ICE {
    #[serde(rename = "Media")]
    pub media: String,
    #[serde(rename = "Index")]
    pub index: i32, 
    #[serde(rename = "Name")]
    pub name: String,
}
impl ICE {
    pub fn new(media: String, index: i32, name: String) -> Self {
        Self{media, index, name}
    }
}