use serde::{Deserialize, Serialize};
use super::{packet::PacketBody, PeerID};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserJSON<Answer, Offer> {
    #[serde(rename = "Destination")]
    pub destination: PeerID, 
    #[serde(rename = "Type")]
    pub r#type: PacketBody<Answer, Offer>,
}