//! maybe go go to 
//! struct Packet {
//!     source: PeerID,
//!     destination: PeerID,
//!     type: PacketType,
//! }
//! 
//! enum PacketType {
//!     ...
//! }
//! 
//! then it would serializes to 
//! {
//!     "Destination": my_id, "Source": source, "Type": {
//!         "Offer": {
//!             "OfferID" : var offer_id, "Offer" : var offer, "ICE" : var ice,
//!         }
//!     }
//! }:
use serde::{Deserialize, Serialize};
use crate::ChannelID;

use super::{OfferID, PeerID, ICE};


#[derive(Serialize, Deserialize, Debug)]
pub struct Packet<Answer, Offer> {
    #[serde(rename = "Source")]
    pub source: PeerID, 
    #[serde(rename = "Destination")]
    pub destination: PeerID, 
    #[serde(rename = "Type")]
    pub r#type: PacketType<Answer, Offer>,
}


#[derive(Serialize, Deserialize, Debug)]
pub enum PacketType<Answer, Offer> {
    Answer{
        #[serde(rename = "Answer")]
        answer: Answer,
        #[serde(rename = "OfferID")]
        offer_id: OfferID,
        #[serde(rename = "ICE")]
        ice: Vec<ICE>,
    },
    Offer{
        #[serde(rename = "Offer")]
        offer: Offer,
        #[serde(rename = "OfferID")]
        offer_id: OfferID,
        #[serde(rename = "ICE")]
        ice: Vec<ICE>,
    },
    InvalidPacket,
    Goodbye,
    NewICE{
        #[serde(rename = "ChannelID")]
        channel_id: ChannelID,
        #[serde(rename = "ICE")]
        ice: ICE,
    }
}

pub fn main(){
    let inner = PacketType::<String, String>::Answer { answer: "spam".into(), offer_id: 69, ice: Vec::from([ICE::new("ham".into(), 2, "sausage".into())]) };
    //let outer = Outer::Answer(inner);
    let packet = Packet{source: "Source".into(), destination: "Destination".into(),r#type: inner};
    println!("\n{}\n", serde_json::to_string(&packet).unwrap());
    println!("{}\n", serde_json::to_string(&packet).unwrap());

}