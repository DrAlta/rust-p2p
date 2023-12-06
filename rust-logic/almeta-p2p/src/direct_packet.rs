use serde::{Deserialize, Serialize};

use crate::routing_entry::RoutingCost;

use super::PeerID;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "Body")]
pub enum DirectPacket {
    DearJohn,
    Goodbye,
    Greetings{
        #[serde(rename = "Me")]
        me: PeerID,
        #[serde(rename = "SupportedVersions")]
        supported_versions: Vec<String>,
    },
    InvalidPacket,
    InvalidSalutation,
    Me{
        #[serde(rename = "Me")]
        me: PeerID,
    },
    NotYouAgain,
    /// source is the first item in trace.
    RouteTraceFromOriginatorToTarget{target: PeerID, trace: Vec<PeerID>},
    /// destination is the first item in trace.
    RouteTraceToOriginatorFromTarget{originator: PeerID, trace: Vec<PeerID>},
    /// this is the sending node's routing cost, you need to add 1 to the cost to get the value you would add to your routing table
    RoutingInformationExchange(Vec::<(PeerID, RoutingCost)>),
    TellItToMeIn(String),
    UnknownVersion,
    Who,
}

pub fn main(){
    let greetings = DirectPacket::Greetings{me: "Spam".into(), supported_versions: Vec::from(["test:0.0".into()])};
    println!("\n{}\n", serde_json::to_string(&greetings).unwrap());
}