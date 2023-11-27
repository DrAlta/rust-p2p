use std::collections::HashMap;

use super::{ChannelID, Command, ICE, Offer, packet::DirectPacket, Packet, PeerID, Answer};

#[derive(Debug)]
struct Outgoing {
    pub user: bool,
    pub answer: Option<Answer>,
    pub ice: Vec<ICE>,
}

impl Outgoing{
    pub fn new(answer: Option<Answer>, ice: Vec<ICE>, user: bool) -> Self {
        Self { answer, ice, user }
    }
}
#[derive(Debug)]
struct Incoming {
    pub user: bool,
    pub offer: Option<Offer>,
    pub ice: Vec<ICE>,
}

impl Incoming{
    pub fn new(offer: Option<Offer>, ice: Vec<ICE>, user: bool) -> Self {
        Self { offer, ice, user}
    }
}


#[derive(Debug)]
pub struct Node {
    pub command_queue: Vec<Command>,

    pub my_id: PeerID,

    neighbors: HashMap<PeerID, ChannelID>,
    
    channel_id_generator_state: i8,

    incoming: HashMap<ChannelID, Incoming>,
    outgoing: HashMap<ChannelID, Outgoing>,
}

/// Creation
impl Node {
    pub fn new(my_id: String) -> Self {
        Self {
            command_queue: Vec::new(), 
            my_id, neighbors: HashMap::new(), 
            channel_id_generator_state: 0, 
            incoming: HashMap::new(),
            outgoing: HashMap::new(),
        }
    }
}

impl Node {
    /*
    pub fn generate_offer(&mut self) -> ChannelID {
        self.channel_id_generator_state += 1;
        let channel_id: ChannelID = self.channel_id_generator_state.into();
        self.command_queue.borrow_mut().push(Command::GenerateOffer(channel_id.clone()));
        channel_id
    }
    */
    pub fn channel_established(&mut self, channel_id: &ChannelID) {
        self.incoming.remove(channel_id);
        self.outgoing.remove(channel_id);
  
        self.send_direct(channel_id.clone(), DirectPacket::Greetings { me: self.my_id.clone(), version: "Rust:0.0".into() })
    }
    pub fn generate_offer(&mut self, user: bool) -> ChannelID {
        self.channel_id_generator_state += 1;
        let channel_id: ChannelID = self.channel_id_generator_state.into();
        self.incoming.insert(channel_id.clone(), Incoming::new(None, Vec::new(), user));
        self.command_queue.push(Command::GenerateOffer(channel_id.clone()));
        channel_id
    }
    /*
    pub fn generate_offer(&mut self) -> ChannelID{
        self.g
        self.incoming.insert(k, v)
        self.command_queue.borrow_mut().push(Command::GenerateOffer);
    }
    */
    pub fn on_offer_generated(&mut self, channel_id: &ChannelID, offer: Offer) {
        let Some(incoming) = self.incoming.get_mut(channel_id) else {
            logy!("tracenode", "incoming {channel_id:?} not found");
            return
        };
        if incoming.user {
            logy!("tracenode", " recieved offer for user on channel {channel_id:?}");
            self.command_queue.push(Command::UserOffer { channel_id: channel_id.clone(), offer: offer.clone() });
        }
        incoming.offer = Some(offer)
    }
    pub fn on_generated_answer(&mut self, channel_id: ChannelID, answer: Answer) {
        logy!("tracenode", "node {} got back answer [{:?}] for channel {}", self.my_id, answer, channel_id);
        let Some(outgoing) = self.outgoing.get_mut(&channel_id) else {
            logy!("trace", "coundn't find outgoing {channel_id}");
            return
        };
        if outgoing.user {
            self.command_queue.push(Command::UserAnswer { channel_id: channel_id.clone(), answer: answer.clone() });
        }
        outgoing.answer = Some(answer);
    }
    pub fn recieve_direct(&mut self, channel_id: &ChannelID, packet: DirectPacket) {
        match packet {
            DirectPacket::Greetings { me, version } => {
                if version != "Rust:0.0" {
                    self.send_direct(channel_id.clone(), DirectPacket::UnknownVersion);
                    return
                }
                if self.neighbors.get(&me).is_some() {
                    self.send_direct(channel_id.clone(), DirectPacket::NotYouAgain)
                } else {
                    self.neighbors.insert(me, channel_id.clone());
                }55
            },
            DirectPacket::Me { .. } => todo!(),
            DirectPacket::Who => todo!(),
            DirectPacket::UnknownVersion => todo!(),
            DirectPacket::NotYouAgain => todo!(),
            DirectPacket::InvalidPacket => todo!(),
            DirectPacket::InvalidSalutation => todo!(),
            DirectPacket::Goodbye => todo!(),
        };
    }
    pub fn recieve_packet(&self, channel_id: &ChannelID, packet: Packet) {
        todo!("{channel_id:?}, {packet:?}")
    }
    pub fn select_channel(&self, peer_id: &PeerID) -> Option<ChannelID> {
        self.neighbors.get(peer_id).cloned()
    }
    pub fn send_direct(&mut self, channel_id: ChannelID, packet: DirectPacket) {
        self.command_queue.push(Command::SendDirect { channel_id, packet });
    }
    pub fn send_packet(&mut self, packet: Packet) {
        let Some(channel_id) = self.select_channel(&packet.destination) else {
            logy!("error", "Couldn't find next node to {}", packet.destination);
            return
        };
        self.command_queue.push(Command::Send { channel_id, packet });
    }
}

impl Node {
    pub fn recieve_offer(&mut self, offer: Offer, user: bool) -> ChannelID {
        self.channel_id_generator_state += 1;
        let channel_id: ChannelID = self.channel_id_generator_state.into();
        self.outgoing.insert(channel_id, Outgoing::new(None, Vec::new(), user));
        self.command_queue.push(Command::GenerateAnswer{channel_id: channel_id.clone(), offer});
        channel_id
    }
   pub fn recieve_answer(&mut self, channel_id: ChannelID, answer: Answer) {
    logy!("trace", "{:?} recieved answer", self.my_id);
    self.command_queue.push(Command::AnswerOffer { channel_id, answer });

    }
}