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
use super::{OfferID, PeerID, ICE};


#[derive(Serialize, Deserialize, Debug)]
pub struct Packet {
    #[serde(rename = "Source")]
    pub source: PeerID, 
    #[serde(rename = "Destination")]
    pub destination: PeerID, 
    #[serde(rename = "Type")]
    pub r#type: PacketType,
}


#[derive(Serialize, Deserialize, Debug)]
pub enum PacketType {
    Answer{
        #[serde(rename = "Answer")]
        answer: String,
        #[serde(rename = "OfferID")]
        offer_id: OfferID,
        #[serde(rename = "ICE")]
        ice: Vec<ICE>,
    },
    Offer{
        #[serde(rename = "Offer")]
        offer: String,
        #[serde(rename = "OfferID")]
        offer_id: OfferID,
        #[serde(rename = "ICE")]
        ice: Vec<ICE>,
    },
    Me{
        #[serde(rename = "Me")]
        me: PeerID,
    },
    Who,
    InvalidPacket,
    Goodbye,
    NewICE{
        #[serde(rename = "OfferID")]
        offer_id: OfferID,
        #[serde(rename = "ICE")]
        ice: Vec<ICE>,
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub enum DirectPacket {
    Greetings{
        #[serde(rename = "Me")]
        me: PeerID,
        #[serde(rename = "Version")]
        version: String,
    },
    Me{
        #[serde(rename = "Me")]
        me: PeerID,
    },
    Who,
    UnknownVersion,
    NotYouAgain,
    InvalidPacket,
    InvalidSalutation,
    Goodbye,
}

pub fn main(){
    let inner = PacketType::Answer { answer: "spam".into(), offer_id: "eggs".into(), ice: Vec::from([ICE::new("ham".into(), 2, "sausage".into())]) };
    //let outer = Outer::Answer(inner);
    let packet = Packet{source: "Source".into(), destination: "Destination".into(),r#type: inner};
    println!("\n{}\n", serde_json::to_string(&packet).unwrap());
    let packet = Packet{source: "Source".into(), destination: "Destination".into(),r#type: PacketType::Who};
    println!("{}\n", serde_json::to_string(&packet).unwrap());

}