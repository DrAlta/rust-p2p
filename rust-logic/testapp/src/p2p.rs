use serde::{Deserialize, Serialize};
use std::{collections::{HashMap, VecDeque}, cell::RefCell};
pub use qol::logy;
use almeta_p2p::{Command, Packet, Node, direct_packet::DirectPacket, LinkID, aux::OfferID};


//pub type Offer = String;


#[derive(Debug)]
enum Message<Answer, Offer> {
    Packet(Packet<Answer, Offer>),
    DirectPacket(DirectPacket),
}

#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct Connection {
    pub peer_idx: usize,
    pub link_id: LinkID,
}
impl Connection {
    pub fn new(peer_idx: usize, link_id: LinkID) -> Self {
        Self {peer_idx, link_id}
    }
}
#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct Answer {
    pub offering_peer_idx: usize,
    pub offering_link_id: LinkID,
    pub answering_peer_idx: usize,
    pub answering_link_id: LinkID,
}
impl Answer {
    pub fn new(offering_peer_idx: usize, offering_link_id: LinkID, answering_peer_idx: usize, answering_link_id: LinkID) -> Self {
        Self {offering_link_id, offering_peer_idx, answering_link_id,answering_peer_idx}
    }
}


#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct Offer {
    pub offering_peer_idx: usize,
    pub offering_link_id: LinkID,
}
impl Offer {
    pub fn new(offering_peer_idx: usize, offering_link_id: LinkID) -> Self {
        Self {offering_link_id, offering_peer_idx}
    }
}

const NUMBER_OF_PEERS: usize = 3;

struct NetworkSim {
    peers: [RefCell<Node<Answer, Offer>>; NUMBER_OF_PEERS],
    peers_channel_to_link: [HashMap<LinkID, Connection>; NUMBER_OF_PEERS],
    link_id_gen_state: [i8;NUMBER_OF_PEERS],

    user_offers_to_be_routed: Vec::<(usize, Offer, OfferID)>,

    answered_queue: Vec::<Answer>,

    /*
    peer_a_user_offer_link_id: (LinkID, OfferID),
    peer_b_user_offer_link_id: (LinkID, OfferID),
    */

}
impl  NetworkSim {
    pub fn init() -> Self {
        let peers= [
            RefCell::new(Node::new("PeerA".into(), 1)),
            RefCell::new(Node::new("PeerB".into(), 2)),
            RefCell::new(Node::new("PeerC".into(), 3)),
        ];
        let _peer_a_user_offer_link_id = peers[0].borrow_mut().generate_offer(None);
        let _peer_b_user_offer_link_id = peers[1].borrow_mut().generate_offer(None);        
        Self {
            peers,
            peers_channel_to_link: [
                HashMap::new(),
                HashMap::new(),
                HashMap::new(),
            ],
            link_id_gen_state: [0_i8;NUMBER_OF_PEERS],
            user_offers_to_be_routed: Vec::<(usize, Offer, OfferID)>::new(),
        
            answered_queue: Vec::<Answer>::new(),
        /*
            peer_a_user_offer_link_id,
            peer_b_user_offer_link_id,
        */
        }
    }
    pub fn sim(&mut self) {
        for i in 0..10 {
            let mut continue_ka = false;
            println!("    Round {i}:");
    
            let mut message_queue= Vec::new();
            for (peer_idx, peer_refcell) in self.peers.iter().enumerate() {
    
                let mut peer = peer_refcell.borrow_mut();
    
                logy!("trace", "processing {} commands of peer:{}", peer.command_queue.len(), peer_idx);
    
                let mut command_queue = VecDeque::new();
                std::mem::swap(&mut peer.command_queue, &mut command_queue);
    
                message_queue = command_queue.into_iter()
                .fold(message_queue, 
                    |mut x, command| {
                        continue_ka = true;
                        println!("Peer {peer_idx} Command: {command}");
                        match command {
                            Command::AddICE { .. } => todo!(),
                            Command::AnswerOffer { link_id: _ , answer } => {
                                /*
                                we should add the peer from the answer into peers_channel_to_link[peer_id].insert(link_id, Link::new(answering_peer, answering_peers_link_id))
                                but we took care of that when generating the answer
                                */
                                self.answered_queue.push(answer)
                                
                                
                            },
                            Command::GenerateAnswer { link_id, offer } => {
                             
                                self.peers_channel_to_link[offer.offering_peer_idx].insert(offer.offering_link_id.clone(), Connection::new( peer_idx.clone(), link_id.clone()));
                                self.peers_channel_to_link[peer_idx.clone()].insert(link_id.clone(), Connection::new(offer.offering_peer_idx, offer.offering_link_id.clone()));
                                let answer = Answer::new(offer.offering_peer_idx, offer.offering_link_id, peer_idx.clone(), link_id.clone());
                                peer.on_answer_generated(&link_id, answer);
                            },
                            Command::GenerateOffer(link_id) => {
                                println!("incrementing peer{peer_idx}'s link gen");
                                self.link_id_gen_state[peer_idx] += 1;
                                //let link_id: LinkID = link_id_gen_state[peer_idx].into();
                                let new_offer= Offer::new(peer_idx.clone(), link_id.clone());
                                //let new_link= Connection::new(peer_idx.clone(), link_id.clone());
                                //incoming_connections.insert(format!("{new_offer:?}"), new_link);
    
                                peer.on_offer_generated(&link_id, new_offer);
    
                            },
                            Command::Send { link_id, packet } => {x.push((
                                self.peers_channel_to_link[peer_idx][&link_id].clone(),
                                Message::Packet(packet)
                            ))},
                            Command::SendDirect { link_id, packet } => x.push((
                                {
                                    let x = &self.peers_channel_to_link[peer_idx];
                                    //println!("{x:#?}");
                                    x[&link_id].clone()
                                },
                                Message::DirectPacket(packet)
                            )),
                            Command::UserAnswer { link_id: _, answer } => {
                                let prev_peer_idx = (NUMBER_OF_PEERS + peer_idx - 1) % NUMBER_OF_PEERS;
                                println!("send answer to peer {prev_peer_idx}");
                                self.peers[prev_peer_idx].borrow_mut().receive_answer(answer.offering_link_id.clone(), answer);
    
    
                            },
                            Command::UserOffer { offer, link_id } => {
                                let next_peer_idx = (peer_idx + 1) % NUMBER_OF_PEERS;
                                self.user_offers_to_be_routed.push((next_peer_idx, offer, link_id.to_inner().into()));
                                
                            },
                        };
                        x
                    }
                );
            }
    //      for Link { peer_idx, link_id } in answered_queue {
            for Answer { offering_peer_idx, offering_link_id, answering_peer_idx, answering_link_id} in std::mem::replace(&mut self.answered_queue, Vec::new()) {
                    continue_ka = true;
                println!("notifing peer at {offering_link_id} of incoming channel establishment");
                self.peers[offering_peer_idx].borrow_mut().link_established(&offering_link_id);
                println!("notifing peer at {answering_link_id} of outgoing channel establishment");
                self.peers[answering_peer_idx].borrow_mut().link_established(&answering_link_id);
            }
            for (peer_idx, offer, offer_id) in std::mem::replace(&mut self.user_offers_to_be_routed, Vec::new()) {
                println!("notifing {peer_idx} of offer");
                continue_ka = true;
                let mut peer = self.peers[peer_idx].borrow_mut();
                peer.receive_offer(offer, offer_id.into(), None);
            }
            for (Connection{peer_idx, link_id}, message) in message_queue {
                continue_ka = true;
                println!("Peer {peer_idx} Message: {:?}", message);
                match message {
                    Message::Packet(packet) => self.peers[peer_idx].borrow_mut().receive_packet(&link_id, packet),
                    Message::DirectPacket(packet_type) => self.peers[peer_idx].borrow_mut().receive_direct(&link_id, packet_type),
                }
            }
            if !continue_ka {
                println!("Done after {i} rounds");
                break;
            }
        }
        }
    
}

#[allow(dead_code)]
pub fn main() {
    let mut sim = NetworkSim::init();
    sim.sim();
    sim.peers[1].borrow_mut().tick();
    sim.sim();
    for (idx, x) in sim.peers.into_iter().enumerate() {
        println!("Peer {idx}: {:?}", x.borrow().get_neighbors());
    }
}