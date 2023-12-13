use serde::{Deserialize, Serialize};

use crate::routing_entry::RoutingCost;

use super::PeerID;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
//#[serde(tag = "Body")]
pub enum DirectBody {
    DearJohn,
    DistanceIncrease{peer: PeerID, trace: Vec<PeerID>},
    Goodbye,
    Greetings{
        #[serde(rename = "Me")]
        me: PeerID,
        #[serde(rename = "SupportedVersions")]
        supported_versions: Vec<String>,
    },
    InvalidPacket,
    InvalidSalutation,
    LostRouteTo{
        #[serde(rename = "Peer")]
        peer: PeerID
    },
    Me{
        #[serde(rename = "Me")]
        me: PeerID,
    },
    NotYouAgain,
    /// source is the first item in trace.
    RouteTraceFromOriginatorToTarget{
        #[serde(rename = "Target")]
        target: PeerID, 
        #[serde(rename = "Trace")]
        trace: Vec<PeerID>
    },
    /// destination is the first item in trace.
    RouteTraceToOriginatorFromTarget{
        #[serde(rename = "Originator")]
        originator: PeerID, 
        #[serde(rename = "trace")]
        trace: Vec<PeerID>
    },
    /// this is the sending node's routing cost, you need to add 1 to the cost to get the value you would add to your routing table
    RoutingInformationExchange{
        #[serde(rename = "Entries")]
        entries: Vec::<(PeerID, RoutingCost)>
    },
    TellItToMeIn{
        #[serde(rename = "Version")]
        version: String
    },
    UnknownVersion,
    Who,
}

impl DirectBody {
    pub fn to_canonical_form(&self) -> String {
        match self {
            DirectBody::DearJohn => {
                "Direct:DearJohn".into()
            },
            DirectBody::DistanceIncrease { peer, trace } => {
                format!("Direct:DistanceIncrease:{peer}:{trace:?}")
            },
            DirectBody::Goodbye => {
                "Direct:Goodbye".into()
            },
            DirectBody::Greetings { me, supported_versions } => {
                format!("Direct:Greetings:{me}:{supported_versions:?}")
            },
            DirectBody::InvalidPacket => {
                "Direct:InvalidPacket".into()
            },
            DirectBody::InvalidSalutation => {
                "Direct:InvalidSalutation".into()
            },
            DirectBody::LostRouteTo { peer } => {
                format!("Direct:LostRouteTo:{peer}")
            }
            DirectBody::Me { me } => {
                format!("Direct:Me:{me}")
            },
            DirectBody::NotYouAgain => {
                "Direct:NotYouAgain".into()
            },
            DirectBody::RouteTraceFromOriginatorToTarget { target, trace } => {
                format!("Direct:RouteTraceFromOriginatorToTarget:{target}:{trace:?}")
            },
            DirectBody::RouteTraceToOriginatorFromTarget { originator, trace } => {
                format!("Direct:RouteTraceToOriginatorFromTarget:{originator}:{trace:?}")
            },
            DirectBody::RoutingInformationExchange { entries } => {
                format!("Direct:RoutingInformationExchange:{entries:?}")
            },
            DirectBody::TellItToMeIn { version } => {
                format!("Direct:TellItToMeIn:{version}")
            },
            DirectBody::UnknownVersion => {
                "Direct:UnknownVersion".into()
            },
            DirectBody::Who => {
                "Direct:Who".into()
            },
        }
    }
    pub fn checksum(&self) -> String {
        format!(
            "{:x}", 
            md5::compute(
                self.to_canonical_form()
            )
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DirectPacket {
    pub md5: String,
    pub body: DirectBody
}
impl From<DirectBody> for DirectPacket {
    fn from(item: DirectBody) -> Self {
        Self { md5: item.checksum(), body: item }
    }
}

impl DirectPacket {
    pub fn new(body: DirectBody) -> Self {
        let md5 = body.checksum();
        Self { md5, body}
    }
    pub fn varify(&self) -> bool {
        self.md5 == self.body.checksum()
    }
}


pub fn main(){
    let greetings = DirectBody::Greetings{me: "Spam".into(), supported_versions: Vec::from(["test:0.0".into()])};
    println!("\n{}\n", serde_json::to_string(&greetings).unwrap());
}
#[cfg(test)]
mod varify_tests {
    use super::*;

    #[test]
    fn dear_john() {
        let packet: DirectPacket = DirectBody::DearJohn.into();
        assert!(packet.varify());
    }

    #[test]
    fn distance_increase() {
        let packet: DirectPacket = DirectBody::DistanceIncrease{peer: "PeerID".into(), trace: Vec::from(["A".into(), "B".into()])}.into();
        assert!(packet.varify());
    }

    #[test]
    fn goodbye() {
        let packet: DirectPacket = DirectBody::Goodbye.into();
        assert!(packet.varify());
    }

    #[test]
    fn greetings() {
        let packet: DirectPacket = DirectBody::Greetings{
        me: "PeerID".into(),
        supported_versions: Vec::from(["a".into(),"b".into()]),
        }.into();
        assert!(packet.varify());
    }

    #[test]
    fn invalid_packet() {
        let packet: DirectPacket = DirectBody::InvalidPacket.into();
        assert!(packet.varify());
    }
    
    #[test]
    fn invalid_salutation() {
        let packet: DirectPacket = DirectBody::InvalidSalutation.into();
        assert!(packet.varify());
    }
    
    #[test]
    fn lost_route_to() {
        let packet: DirectPacket = DirectBody::LostRouteTo{peer: "PeerID".into()}.into();
        assert!(packet.varify());
    }
    
    #[test]
    fn me() {
        let packet: DirectPacket = DirectBody::Me{
            me: "PeerID".into(),
        }.into();
        assert!(packet.varify());
    }
    
    #[test]
    fn not_you_again() {
        let packet: DirectPacket = DirectBody::NotYouAgain.into();
        assert!(packet.varify());
    }
    
    #[test]
    fn route_trace_from_originator_to_target() {
        let packet: DirectPacket = DirectBody::RouteTraceFromOriginatorToTarget{target: "PeerID".into(), trace: Vec::from(["a".into(), "b".into()])}.into();
        assert!(packet.varify());
    }
    
    #[test]
    fn route_trace_to_originator_from_target() {
        let packet: DirectPacket = DirectBody::RouteTraceToOriginatorFromTarget{originator: "PeerID".into(), trace: Vec::from(["a".into(), "b".into()])}.into();
        assert!(packet.varify());
    }
    
    #[test]
    fn routing_information_exchange() {
        let packet: DirectPacket = DirectBody::RoutingInformationExchange{entries: Vec::from([("a".into(), 1), ("b".into(), 2)])}.into();
        assert!(packet.varify());
    }
    
    #[test]
    fn tell_it_to_me_in() {
        let packet: DirectPacket = DirectBody::TellItToMeIn{version: "V".into()}.into();
        assert!(packet.varify());
    }
    
    #[test]
    fn unknown_version() {
        let packet: DirectPacket = DirectBody::UnknownVersion.into();
        assert!(packet.varify());
    }
    
    #[test]
    fn who() {
        let packet: DirectPacket = DirectBody::Who.into();
        assert!(packet.varify());
    }
}
#[cfg(test)]
mod json_tests {
    use serde_json::{to_string, from_str};
    use super::*;



    #[test]
    fn dear_john() {
        let packet: DirectPacket = DirectBody::DearJohn.into();
        assert_eq!(packet, from_str(&to_string(&packet).unwrap()).unwrap());
    }

    #[test]
    fn distance_increase() {
        let packet: DirectPacket = DirectBody::DistanceIncrease{peer: "PeerID".into(), trace: Vec::from(["A".into(), "B".into()])}.into();
        assert_eq!(packet, from_str(&to_string(&packet).unwrap()).unwrap());
    }

    #[test]
    fn goodbye() {
        let packet: DirectPacket = DirectBody::Goodbye.into();
        assert_eq!(packet, from_str(&to_string(&packet).unwrap()).unwrap());
    }

    #[test]
    fn greetings() {
        let packet: DirectPacket = DirectBody::Greetings{
        me: "PeerID".into(),
        supported_versions: Vec::from(["a".into(),"b".into()]),
        }.into();
        assert_eq!(packet, from_str(&to_string(&packet).unwrap()).unwrap());
    }

    #[test]
    fn invalid_packet() {
        let packet: DirectPacket = DirectBody::InvalidPacket.into();
        assert_eq!(packet, from_str(&to_string(&packet).unwrap()).unwrap());
    }
    
    #[test]
    fn invalid_salutation() {
        let packet: DirectPacket = DirectBody::InvalidSalutation.into();
        assert_eq!(packet, from_str(&to_string(&packet).unwrap()).unwrap());
    }
    
    #[test]
    fn lost_route_to() {
        let packet: DirectPacket = DirectBody::LostRouteTo{peer: "PeerID".into()}.into();
        assert_eq!(packet, from_str(&to_string(&packet).unwrap()).unwrap());
    }
    
    #[test]
    fn me() {
        let packet: DirectPacket = DirectBody::Me{
            me: "PeerID".into(),
        }.into();
        assert_eq!(packet, from_str(&to_string(&packet).unwrap()).unwrap());
    }
    
    #[test]
    fn not_you_again() {
        let packet: DirectPacket = DirectBody::NotYouAgain.into();
        assert_eq!(packet, from_str(&to_string(&packet).unwrap()).unwrap());
    }
    
    #[test]
    fn route_trace_from_originator_to_target() {
        let packet: DirectPacket = DirectBody::RouteTraceFromOriginatorToTarget{target: "PeerID".into(), trace: Vec::from(["a".into(), "b".into()])}.into();
        assert_eq!(packet, from_str(&to_string(&packet).unwrap()).unwrap());
    }
    
    #[test]
    fn route_trace_to_originator_from_target() {
        let packet: DirectPacket = DirectBody::RouteTraceToOriginatorFromTarget{originator: "PeerID".into(), trace: Vec::from(["a".into(), "b".into()])}.into();
        assert_eq!(packet, from_str(&to_string(&packet).unwrap()).unwrap());
    }
    
    #[test]
    fn routing_information_exchange() {
        let packet: DirectPacket = DirectBody::RoutingInformationExchange{entries: Vec::from([("a".into(), 1), ("b".into(), 2)])}.into();
        assert_eq!(packet, from_str(&to_string(&packet).unwrap()).unwrap());
    }
    
    #[test]
    fn tell_it_to_me_in() {
        let packet: DirectPacket = DirectBody::TellItToMeIn{version: "V".into()}.into();
        assert_eq!(packet, from_str(&to_string(&packet).unwrap()).unwrap());
    }
    
    #[test]
    fn unknown_version() {
        let packet: DirectPacket = DirectBody::UnknownVersion.into();
        assert_eq!(packet, from_str(&to_string(&packet).unwrap()).unwrap());
    }
    
    #[test]
    fn who() {
        let packet: DirectPacket = DirectBody::Who.into();
        assert_eq!(packet, from_str(&to_string(&packet).unwrap()).unwrap());
    }
}