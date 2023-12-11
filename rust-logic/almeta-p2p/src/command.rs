use std::fmt;
use serde::{self, Deserialize, Serialize};
use serde_json;

use super::{LinkID, ICE, Packet, DirectPacket};
#[derive(Debug, Deserialize, Serialize)]
pub enum Command<Answer, Offer> {
    AddICE{link_id: LinkID, ice: ICE},
    AnswerOffer{link_id: LinkID, answer: Answer},
    GenerateAnswer{link_id: LinkID, offer: Offer},
    GenerateOffer(LinkID),
    Send{link_id: LinkID, packet: Packet<Answer, Offer>},
    SendDirect{link_id: LinkID, packet: DirectPacket},
    UserAnswer{link_id: LinkID, answer: Answer},
    UserOffer{link_id: LinkID, offer: Offer},
}
impl<'a, 'b, Answer: Deserialize<'a> + Serialize, Offer: Deserialize<'b> + Serialize> fmt::Display for Command<Answer, Offer> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}