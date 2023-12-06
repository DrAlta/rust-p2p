use serde::{Deserialize, Serialize};
use std::{collections::{HashMap, VecDeque}, cell::RefCell};

use almeta_p2p::{Command, Packet, Node, DirectPacket, LinkID, OfferID};


pub type Offer = String;
pub type Answer = Link;


#[derive(Debug)]
enum Message<Answer, Offer> {
    Packet(Packet<Answer, Offer>),
    DirectPacket(DirectPacket),
}

#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct Link {
    pub peer_idx: usize,
    pub link_id: LinkID,
}
impl Link {
    pub fn new(peer_idx: usize, link_id: LinkID) -> Self {
        Self {peer_idx, link_id}
    }
}

const NUMBER_OF_PEERS: usize = 2;

#[allow(dead_code)]
pub fn main() {
    let peers: [RefCell<Node<Answer, Offer>>; NUMBER_OF_PEERS] = [
        RefCell::new(Node::new("PeerA".into())),
        RefCell::new(Node::new("PeerB".into())),
    ];


    let mut peers_channel_to_link: [HashMap<LinkID, Link>; NUMBER_OF_PEERS] = [
        HashMap::new(),
        HashMap::new(),
    ]; 
    let mut link_id_gen_state = [0_i8;NUMBER_OF_PEERS];
    assert_eq!(peers.len(), link_id_gen_state.len());
    let mut incoming_connections = HashMap::<String, Link>::new();

    let _peer_a_user_offer_link_id = peers[0].borrow_mut().generate_offer(true);
    let mut user_offers_to_be_routed = Vec::<(usize, Offer, OfferID)>::new();

    let mut answered_queue =  Vec::<Answer>::new();

    //while !peers.iter().all(|x| x.command_queue.borrow().is_empty());
    for i in 0..10 {
        println!("interating {i}:");

        let mut message_queue= Vec::new();
        for (peer_idx, peer_refcell) in peers.iter().enumerate() {
            let mut peer = peer_refcell.borrow_mut();

            // logy!("trace", "processing {} commands of peer:{} at {}", peer.command_queue.len(), peer.my_id, peer_idx);

            let mut command_queue = VecDeque::new();
            std::mem::swap(&mut peer.command_queue, &mut command_queue);

            message_queue = command_queue.into_iter()
            .fold(message_queue, 
                |mut x, command| {
                    logy!("trace", "processing {command:?}");
                    match command {
                        Command::AddICE { .. } => todo!(),
                        Command::AnswerOffer { link_id, answer } => {
                            /*
                            we should add the peer from the answer into peers_channel_to_link[peer_id].insert(link_id, Link::new(answering_peer, answering_peers_link_id))
                            but we took care of that when generating the answer
                            */
                            peer.channel_established(&link_id);
                            answered_queue.push(answer)
                            
                            
                        },
                        Command::GenerateAnswer { link_id, offer } => {
                            let Some(incoming_link) = incoming_connections.remove(&offer) else {
                                logy!("trace", "failed to find {offer:?}");
                                return x;
                            };
                            peer.on_answer_generated(&link_id, Link::new(peer_idx, link_id));
                            peers_channel_to_link[incoming_link.peer_idx].insert(incoming_link.link_id, Link::new(peer_idx, link_id));
                            peers_channel_to_link[peer_idx].insert(link_id, incoming_link);
                        },
                        Command::GenerateOffer(link_id) => {
                            link_id_gen_state[peer_idx] += 1;
                            //let link_id: LinkID = link_id_gen_state[peer_idx].into();
                            let link_string = format!("P{}:C{}", peer_idx, link_id_gen_state[peer_idx]);
                            let new_link= Link::new(peer_idx, link_id);
                            incoming_connections.insert(link_string.clone(), new_link);

                            peer.on_offer_generated(&link_id, link_string);

                        },
                        Command::Send { link_id, packet } => {x.push((
                            peers_channel_to_link[peer_idx][&link_id].clone(),
                            Message::Packet(packet)
                        ))},
                        Command::SendDirect { link_id, packet } => x.push((
                            peers_channel_to_link[peer_idx][&link_id].clone(),
                            Message::DirectPacket(packet)
                        )),
                        Command::UserAnswer { link_id, answer } => {
                            let prev_peer_idx = (NUMBER_OF_PEERS + peer_idx - 1) % NUMBER_OF_PEERS;
                            peers[prev_peer_idx].borrow_mut().receive_answer(link_id, answer);


                        },
                        Command::UserOffer { offer, link_id } => {
                            let next_peer_idx = (peer_idx + 1) % NUMBER_OF_PEERS;
                            user_offers_to_be_routed.push((next_peer_idx, offer, link_id));
                            
                        },
                    };
                    x
                }
            );
        }
        for Link { peer_idx, link_id } in answered_queue {
            println!("notifing peer at {peer_idx} of outgoing channel establishment");
            peers[peer_idx].borrow_mut().channel_established(&link_id);
        }
        answered_queue = Vec::new();
        for (peer_idx, offer, offer_id) in user_offers_to_be_routed {
            let mut peer = peers[peer_idx].borrow_mut();
            peer.receive_offer(offer, offer_id, true);
        }
        user_offers_to_be_routed = Vec::new();
        for (Link{peer_idx, link_id}, message) in message_queue {
            logy!("trace", "rounting message [{:?}]", message);
            match message {
                Message::Packet(packet) => peers[peer_idx].borrow_mut().receive_packet(&link_id, packet),
                Message::DirectPacket(packet_type) => peers[peer_idx].borrow_mut().receive_direct(&link_id, packet_type),
            }
        }
    }

}