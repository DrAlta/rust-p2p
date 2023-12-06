use super::{LinkID, ICE, Packet, DirectPacket};
#[derive(Debug)]
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