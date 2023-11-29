use serde::{Deserialize, Serialize};
use super::{packet::PacketType, PeerID};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserJSON<Answer, Offer> {
    #[serde(rename = "Destination")]
    pub destination: PeerID, 
    #[serde(rename = "Type")]
    pub r#type: PacketType<Answer, Offer>,
}