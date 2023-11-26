use super::{Answer, ChannelID, ICE, Offer, Packet, packet::DirectPacket};
#[derive(Debug)]
pub enum Command {
    AddICE{channel_id: ChannelID, ice: ICE},
    AnswerOffer{channel_id: ChannelID, answer: Answer},
    GenerateAnswer{channel_id: ChannelID, offer: Offer},
    GenerateOffer(ChannelID),
    Send{channel_id: ChannelID, packet: Packet},
    SendDirect{channel_id: ChannelID, packet: DirectPacket},
    UserAnswer{channel_id: ChannelID, answer: Answer},
    UserOffer{channel_id: ChannelID, offer: Offer},
}