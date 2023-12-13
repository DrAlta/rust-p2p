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
use md5;
use serde::{Deserialize, Serialize};
use crate::LinkID;

use super::{OfferID, PeerID, ICE};


#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Packet<Answer, Offer> {
    #[serde(rename = "Source")]
    pub source: PeerID, 
    #[serde(rename = "Destination")]
    pub destination: PeerID, 
    #[serde(rename = "Body")]
    pub body: PacketBody<Answer, Offer>,
    #[serde(rename = "MD5")]
    pub md5: String,
}
impl<Answer: std::fmt::Debug, Offer: std::fmt::Debug> Packet<Answer, Offer> {
    pub fn new(source:PeerID, destination: PeerID, body: PacketBody<Answer, Offer>) -> Self{
        let canonical_form = format!("Packet:{}:{}:{:?}", source, destination, body.to_canonical_form());
        let md5 = format!(
            "{:x}", 
            md5::compute(
                canonical_form
            )
        );
        Self{ source, destination, body, md5 }
    }
    pub fn to_canonical_form(&self) -> String {
        format!("Packet:{}:{}:{:?}", self.source, self.destination, self.body.to_canonical_form())
    }
    pub fn checksum(&self) -> String {
        format!(
            "{:x}", 
            md5::compute(
                self.to_canonical_form()
            )
        )
    }
    pub fn varify(&self) -> bool {
        self.md5 == self.checksum()
    }
}


#[derive(Serialize, Deserialize, Debug, PartialEq)]
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
    RequestOffer,
    RequestTraceToMe,
    ReturnRouteTrace{trace:Vec<PeerID>},
 //   WhoAreYourNeighbors,
    /* this is for unknown packets
    #[serde(untagged)]
    Json(serde_json::Value),
    */

}
impl<Answer: std::fmt::Debug, Offer: std::fmt::Debug> PacketBody<Answer, Offer> {
    pub fn to_canonical_form(&self) -> String {
        match self {
            PacketBody::Answer { answer, offer_id, ice } => {
                format!("Answer:{answer:?}:{offer_id}:{ice:?}")
            },
            PacketBody::Offer { offer, offer_id, ice } => {
                format!("Offer:{offer:?}:{offer_id}:{ice:?}")
            },
            PacketBody::InvalidPacket => {
                "InvalidPacket".into()
            },
            PacketBody::Goodbye => {
                "Goodbye".into()
            },
            PacketBody::NewICE { link_id, ice } => {
                format!("NewICE:{link_id}:{ice:?}")
            },
            PacketBody::RequestOffer => {
                "RequestOffer".into()
            },
            PacketBody::RequestTraceToMe => {
                "RequestTraceToMe".into()
            },
            PacketBody::ReturnRouteTrace{trace} => {
                format!("ReturnRouteTrace:{trace:?}")
            },
        }
    }
}

pub fn main(){
    let inner = PacketBody::<String, String>::Answer { answer: "spam".into(), offer_id: 69.into(), ice: Vec::from([ICE::new("ham".into(), 2, "sausage".into())]) };
    //let outer = Outer::Answer(inner);
    let packet = Packet::new("Source".into(), "Destination".into(), inner);
    println!("\n{}\n", serde_json::to_string(&packet).unwrap());
    println!("{}\n", serde_json::to_string(&packet).unwrap());

}

#[cfg(test)]
mod varify_tests {
    use super::*;

    #[test]
    fn answer() {
        let body = PacketBody::<String, String>::Answer{
            answer: "Answer".into(),
            offer_id: 1.into(),
            ice: Vec::from([ICE::new("media".into(), 1, "name".into())]),
        };
        let packet = Packet::new("A".into(), "B".into(), body);
        assert!(packet.varify());
    }
        
    #[test]
    fn offer() {
        let body = PacketBody::<String, String>::Offer{
        offer: "Offer".into(),
        offer_id: 1.into(),
        ice: Vec::from([ICE::new("media".into(), 1, "name".into())]),
        };
        let packet = Packet::new("A".into(), "B".into(), body);
        assert!(packet.varify());
    }
        
    #[test]
    fn invalid_packet() {
        let body = PacketBody::<String, String>::InvalidPacket;
        let packet = Packet::new("A".into(), "B".into(), body);
        assert!(packet.varify());
    }
        
    #[test]
    fn goodbye() {
        let body = PacketBody::<String, String>::Goodbye;
        let packet = Packet::new("A".into(), "B".into(), body);
        assert!(packet.varify());
    }
        
    #[test]
    fn new_ice() {
        let body = PacketBody::<String, String>::NewICE{
            link_id: 1.into(),
            ice: ICE::new("media".into(), 1, "name".into()),
        };
        let packet = Packet::new("A".into(), "B".into(), body);
        assert!(packet.varify());
    }
        
    #[test]
    fn request_offer() {
        let body = PacketBody::<String, String>::RequestOffer;
        let packet = Packet::new("A".into(), "B".into(), body);
        assert!(packet.varify());
    }
        
    #[test]
    fn request_trace_to_me() {
        let body = PacketBody::<String, String>::RequestTraceToMe;
        let packet = Packet::new("A".into(), "B".into(), body);
        assert!(packet.varify());
    }
        
    #[test]
    fn return_route_trace() {
        let body = PacketBody::<String, String>::ReturnRouteTrace{
            trace: Vec::from(["A".into(), "B".into()])};
        let packet = Packet::new("A".into(), "B".into(), body);
        assert!(packet.varify());
    }


}

#[cfg(test)]
mod json_tests {
    use serde_json::{to_string, from_str};
    use super::*;

    #[test]
    fn answer() {
        let body = PacketBody::<String, String>::Answer{
            answer: "Answer".into(),
            offer_id: 1.into(),
            ice: Vec::from([ICE::new("media".into(), 1, "name".into())]),
        };
        let packet = Packet::new("A".into(), "B".into(), body);
        assert_eq!(packet, from_str(&to_string(&packet).unwrap()).unwrap());
    }
        
    #[test]
    fn offer() {
        let body = PacketBody::<String, String>::Offer{
        offer: "Offer".into(),
        offer_id: 1.into(),
        ice: Vec::from([ICE::new("media".into(), 1, "name".into())]),
        };
        let packet = Packet::new("A".into(), "B".into(), body);
        assert_eq!(packet, from_str(&to_string(&packet).unwrap()).unwrap());
    }
        
    #[test]
    fn invalid_packet() {
        let body = PacketBody::<String, String>::InvalidPacket;
        let packet = Packet::new("A".into(), "B".into(), body);
        assert_eq!(packet, from_str(&to_string(&packet).unwrap()).unwrap());
    }
        
    #[test]
    fn goodbye() {
        let body = PacketBody::<String, String>::Goodbye;
        let packet = Packet::new("A".into(), "B".into(), body);
        assert_eq!(packet, from_str(&to_string(&packet).unwrap()).unwrap());
    }
        
    #[test]
    fn new_ice() {
        let body = PacketBody::<String, String>::NewICE{
            link_id: 1.into(),
            ice: ICE::new("media".into(), 1, "name".into()),
        };
        let packet = Packet::new("A".into(), "B".into(), body);
        assert_eq!(packet, from_str(&to_string(&packet).unwrap()).unwrap());
    }
        
    #[test]
    fn request_offer() {
        let body = PacketBody::<String, String>::RequestOffer;
        let packet = Packet::new("A".into(), "B".into(), body);
        assert_eq!(packet, from_str(&to_string(&packet).unwrap()).unwrap());
    }
        
    #[test]
    fn request_trace_to_me() {
        let body = PacketBody::<String, String>::RequestTraceToMe;
        let packet = Packet::new("A".into(), "B".into(), body);
        assert_eq!(packet, from_str(&to_string(&packet).unwrap()).unwrap());
    }
        
    #[test]
    fn return_route_trace() {
        let body = PacketBody::<String, String>::ReturnRouteTrace{
            trace: Vec::from(["A".into(), "B".into()])};
        let packet = Packet::new("A".into(), "B".into(), body);
        assert_eq!(packet, from_str(&to_string(&packet).unwrap()).unwrap());
    }


}