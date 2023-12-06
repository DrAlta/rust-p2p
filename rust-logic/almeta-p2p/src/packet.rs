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
use crate::LinkID;

use super::{OfferID, PeerID, ICE};


#[derive(Serialize, Deserialize, Debug)]
pub struct Packet<Answer, Offer> {
    #[serde(rename = "Source")]
    pub source: PeerID, 
    #[serde(rename = "Destination")]
    pub destination: PeerID, 
    #[serde(rename = "Body")]
    pub body: PacketBody<Answer, Offer>,
}


#[derive(Serialize, Deserialize, Debug)]
pub enum PacketBody<Answer, Offer> {
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
    //MyNeighbors(Vec<PeerID>),
    NewICE{
        #[serde(rename = "LinkID")]
        link_id: LinkID,
        #[serde(rename = "ICE")]
        ice: ICE,
    },
    RequestTraceToMe,
    ReturnRouteTrace(Vec<PeerID>),
 //   WhoAreYourNeighbors,
    /* this is for unknown packets
    #[serde(untagged)]
    Json(serde_json::Value),
    */

}

pub fn main(){
    let inner = PacketBody::<String, String>::Answer { answer: "spam".into(), offer_id: 69, ice: Vec::from([ICE::new("ham".into(), 2, "sausage".into())]) };
    //let outer = Outer::Answer(inner);
    let packet = Packet{source: "Source".into(), destination: "Destination".into(),body: inner};
    println!("\n{}\n", serde_json::to_string(&packet).unwrap());
    println!("{}\n", serde_json::to_string(&packet).unwrap());

}