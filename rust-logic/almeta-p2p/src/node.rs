use std::collections::{HashMap, VecDeque};
use crate::{logy, OfferID, packet::PacketBody, unwrap_or_return};

use super::{LinkID, Command, ICE, DirectPacket, Packet, PeerID, routing_entry::{RoutingCost, RoutingEntry}, Incoming, Outgoing};

const PROTOCAL_VERSION: &str = concat!("Rust", ":", "0.1");

#[derive(Debug)]
pub struct Node<Answer, Offer> {
    pub command_queue: VecDeque<Command<Answer, Offer>>,

    pub my_id: PeerID,

    neighbors: HashMap<PeerID, LinkID>,

    routing_table: HashMap<PeerID, RoutingEntry>,
    
    link_id_generator_state: i8,

    incoming: HashMap<LinkID, Incoming<Offer>>,
    outgoing: HashMap<LinkID, Outgoing<Answer>>,
}

/// Creation
impl<Answer, Offer> Node<Answer, Offer> {
    pub fn new(my_id: String) -> Self {
        let my_id = my_id.into();
        Self {
            command_queue: VecDeque::new(), 
            my_id, 
            neighbors: HashMap::from([("User".into(), 0)]),
            routing_table: HashMap::from([("User".into(), RoutingEntry::new(0,0))]),
            link_id_generator_state: 0, 
            incoming: HashMap::new(),
            outgoing: HashMap::new(),
        }
    }
}

impl<Answer: Clone + serde::Serialize, Offer: Clone + serde::Serialize> Node<Answer, Offer> {
    /*
    pub fn generate_offer(&mut self) -> LinkID {
        self.link_id_generator_state += 1;
        let link_id: LinkID = self.link_id_generator_state.into();
        self.command_queue.borrow_mut().push(Command::GenerateOffer(link_id.clone()));
        link_id
    }
    */
    pub fn channel_closed(&mut self, link_id: &LinkID) {
        self.neighbors.retain(|_peer_id, test_link_id| test_link_id != link_id);
        self.incoming.remove(link_id);
        self.outgoing.remove(link_id);
    }
    pub fn channel_established(&mut self, link_id: &LinkID) {
        self.incoming.remove(link_id);
        self.outgoing.remove(link_id);
  
        self.send_direct(link_id.clone(), DirectPacket::Greetings { me: self.my_id.clone(), supported_versions: Vec::from([PROTOCAL_VERSION.into()]) })
    }
    pub fn get_answer_json_by_id(&self, link_id: &LinkID) -> Option<String> {
        let outgoing = self.outgoing.get(link_id)?;
        let answer = outgoing.answer.clone()?;
        let inner = PacketBody::<Answer, Offer>::Answer { 
            answer: answer, 
            offer_id: outgoing.offer_id.clone(), 
            ice: outgoing.ice.clone()
        };
        let user = Packet{
            source: "User".into(),
            destination: self.my_id.clone(),
            body: inner
        };
        serde_json::to_string(&user).ok()
    }
    pub fn get_offer_json_by_id(&self, link_id: &LinkID) -> Option<String> {
        let incoming = self.incoming.get(link_id)?;
        let offer = incoming.offer.clone()?;
        let inner = PacketBody::<Answer, Offer>::Offer { 
            offer: offer, 
            offer_id: link_id.clone(),
            ice: incoming.ice.clone()
        };
        let user = Packet{
            source: "User".into(),
            destination: self.my_id.clone(),
            body: inner
        };
        serde_json::to_string(&user).ok()
    }
    pub fn generate_offer(&mut self, user: bool) -> LinkID {
        self.link_id_generator_state += 1;
        let link_id: LinkID = self.link_id_generator_state.into();
        self.incoming.insert(link_id.clone(), Incoming::new(None, Vec::new(), user));
        self.command_queue.push_back(Command::GenerateOffer(link_id.clone()));
        link_id
    }
    /*
    pub fn generate_offer(&mut self) -> LinkID{
        self.g
        self.incoming.insert(k, v)
        self.command_queue.borrow_mut().push(Command::GenerateOffer);
    }
    */
    pub fn on_answer_generated(&mut self, link_id: &LinkID, answer: Answer) {
        logy!("tracenode", "node {} got back answer [{:?}] for channel {}", self.my_id, answer, link_id);
        let outgoing = unwrap_or_return!(
            self.outgoing.get_mut(link_id),
            logy!("trace", "coundn't find outgoing {link_id}"),
            ()
        );
        if outgoing.user {
            self.command_queue.push_back(Command::UserAnswer { link_id: link_id.clone(), answer: answer.clone() });
        }
        outgoing.answer = Some(answer);
    }
    pub fn on_offer_generated(&mut self, link_id: &LinkID, offer: Offer) {
        let incoming = unwrap_or_return!(
            self.incoming.get_mut(link_id),
            logy!("tracenode", "incoming {link_id:?} not found"),
            ()   
        );
        if incoming.user {
            logy!("tracenode", " received offer for user on channel {link_id:?}");
            self.command_queue.push_back(Command::UserOffer { link_id: link_id.clone(), offer: offer.clone() });
        }
        incoming.offer = Some(offer)
    }
    pub fn receive_direct(&mut self, link_id: &LinkID, packet: DirectPacket) {
        match packet {
            DirectPacket::DearJohn => {
                let peer_id = unwrap_or_return!( self.get_peer_id_from_link_id(link_id));
                if ! self.is_keeper(&peer_id) {
                    self.send_direct(link_id.clone(), DirectPacket::Goodbye);
                }
            },
            DirectPacket::DistanceIncrease { peer, mut trace } => {
                let next_hop = unwrap_or_return!(self.get_next_hop_to(&peer));
                if link_id == &next_hop {
                    // check if we in the trace;
                    if trace.contains(&self.my_id) {
                        let packet= DirectPacket::LostRouteTo(peer);
                        self.direct_broadcast(None, packet);
                        return
                    }
                    trace.push(self.my_id.clone());
                    self.routing_table.insert(peer.clone(), RoutingEntry::new(next_hop, trace.len() as u32));
                    let packet= DirectPacket::DistanceIncrease { peer, trace};
                    self.direct_broadcast(Some(link_id), packet);
                }
                
            },
            DirectPacket::Goodbye => todo!(),
            DirectPacket::Greetings { me, supported_versions } => {
                let version = PROTOCAL_VERSION.into();
                if supported_versions.contains(&version) {
                    if self.neighbors.get(&me).is_some() {
                        self.send_direct(link_id.clone(), DirectPacket::NotYouAgain)
                    } else {
                        self.neighbors.insert(me, link_id.clone());
                        self.send_direct(link_id.clone(), DirectPacket::TellItToMeIn(version))
                    }
                } else {
                    self.send_direct(link_id.clone(), DirectPacket::UnknownVersion);
                }
            },
            DirectPacket::InvalidPacket => todo!(),
            DirectPacket::InvalidSalutation => todo!(),
            DirectPacket::LostRouteTo(peer) => {
                let RoutingEntry { next_hop, routing_cost } = unwrap_or_return!(self.routing_table.get(&peer));
                if next_hop == link_id {
                    let packet= DirectPacket::LostRouteTo(peer);
                    self.direct_broadcast(None, packet);
                } else {
                    let packet = DirectPacket::RoutingInformationExchange(
                        Vec::from(
                            [
                                (peer, routing_cost.clone())
                            ]
                        )
                    );
                    self.send_direct(link_id.clone(), packet);
                }
            },
            DirectPacket::Me { .. } => {

            },
            DirectPacket::NotYouAgain => {
                // ToDo
                ()
            },
            DirectPacket::RouteTraceFromOriginatorToTarget { target, mut trace } => {
                let originator = unwrap_or_return!(trace.get(0));
                if &self.my_id == &target {
                    let target = originator.clone();
                    let packet_body = PacketBody::ReturnRouteTrace(trace);
                    self.send_to(target, packet_body);
                    return
                }
                let next_hop = unwrap_or_return!(self.get_next_hop_to(&target));
                trace.push(self.my_id.clone());
                let new_packet = DirectPacket::RouteTraceFromOriginatorToTarget{
                    target, 
                    trace
                };
                self.send_direct(next_hop, new_packet)
            },
            DirectPacket::RouteTraceToOriginatorFromTarget { originator, mut trace } => {
                let _target = unwrap_or_return!(
                    trace.get(0),
                    logy!("networkerror", "recieved empty RouteTraceToSourceFromDestination for {originator}, nothin we can do about it"),
                    ()
                );
                if &originator == &self.my_id {
                    // this trace was emnt for us so handle it
                    return
                    /*
                    let Some(last_hop) = trace.last() else {
                        logy!("networkingerror", "the trace is empty but that is impossible as we know then is atleast the source in the trace.");
                        return
                    };
                    if let Some(next_hop) = self.get_next_hop_to(destination) {
                        if link_id == &next_hop {

                        }
                    } else {
                        self.routing_table.insert(destination.clone(), RoutingEntry::new(link_id.clone(), trace.len() as u32))
                    };
                    */
                } else {
                    let next_hop = unwrap_or_return!(self.get_next_hop_to(&originator));
                    trace.push(self.my_id.clone());
                    let new_packet = DirectPacket::RouteTraceToOriginatorFromTarget{
                        originator, 
                        trace
                    };
                    self.send_direct(next_hop, new_packet)
                }
            },

            DirectPacket::RoutingInformationExchange(entries) => {
                self.update_routing_table(link_id, entries);
            },
            DirectPacket::TellItToMeIn(version) => {
                // Todo need to store this the use it when talking to them.
                if &version != PROTOCAL_VERSION {
                    self.send_direct(link_id.clone(), DirectPacket::UnknownVersion);
                }
            }
            DirectPacket::UnknownVersion => todo!(),
            DirectPacket::Who => {
                self.send_direct(link_id.clone(), DirectPacket::Me { me: self.my_id.clone() });
            },
        };
    }
    pub fn receive_packet(&mut self, link_id: &LinkID, packet: Packet<Answer, Offer>) {
        if packet.destination == self.my_id {
            match packet.body {
                PacketBody::Answer { answer, offer_id, ice} => {
                    self.command_queue.push_back(Command::AnswerOffer { link_id: offer_id, answer });
                    for icee in ice {
                        self.command_queue.push_back(Command::AddICE { link_id: offer_id.clone(), ice: icee});
                    };
                },
                PacketBody::Goodbye => todo!(),
                PacketBody::InvalidPacket => {
                    logy!("error", "Got a reply that I sent a invalid packet");
                },
                PacketBody::Offer {offer, offer_id, ice} => {
                    let id = self.receive_offer(offer, offer_id, packet.source == "User".into());
                    for icee in ice {
                        self.command_queue.push_back(Command::AddICE { link_id: id.clone(), ice: icee});
                    };
                },
/* Decided to go with a diffrant route :P
                PacketBody::MyNeighbors(_) => {
                },
*/
                PacketBody::NewICE { link_id, ice } => {
                    let incoming = unwrap_or_return!(
                        self.incoming.get_mut(&link_id),
                        logy!("error", "couldn't find an incoming for {}", link_id),
                        ()
                    );
                    incoming.ice.push(ice.clone());
                    //self.command_queue.push_back(Command::AddICE { link_id, ice});
                    /*
                    for ice in ice {
                        self.command_queue.push_back(Command::AddICE { link_id: link_id.clone(), ice});
                    }
                    */

                },
                PacketBody::RequestTraceToMe => {
                    let next_hop =  unwrap_or_return!(self.get_next_hop_to(&packet.source));

                    let new_packet = DirectPacket::RouteTraceToOriginatorFromTarget{
                        originator: packet.source, 
                        trace: Vec::from([self.my_id.clone()])
                    };
                    self.send_direct(next_hop, new_packet)
                },
                PacketBody::ReturnRouteTrace(mut trace) => {
                    if Some(&self.my_id) == trace.get(0) {
                        // it was a trace from us to `packet.source`
                        let first_hop = unwrap_or_return!(trace.get(1));
                        if let Some(next_hop) = self.get_next_hop_to(&packet.source) {
                            let next_hop_peer_id = unwrap_or_return!(
                                self.get_peer_id_from_link_id(&next_hop),
                                logy!("networkingerror", "if we are routing thou a link then we should be a link to a neighbor"),
                                ()
                            );
                            if &next_hop_peer_id == first_hop {
                                // check if we in the trace;
                                //first remove ourselve from the trace.
                                trace.swap_remove(0);
                                if trace.contains(&self.my_id) {
                                    // it was a trace from us, and
                                    // it went throu node that I current route to the target and
                                    // it goes through me
                                    // therefore
                                    // clear are routing to target and announce it
                                    self.routing_table.remove_entry(&packet.source);
                                    let packet= DirectPacket::LostRouteTo(packet.source.clone());
                                    self.direct_broadcast(None, packet);
                                    return
                                }
                                self.routing_table.insert(
                                    packet.source.clone(), 
                                    RoutingEntry::new(
                                        next_hop, 
                                        // add one to the cost because the tace does include the ending peer(it can be find from the packet source) and re removed oursef from it
                                        trace.len() as u32 + 1
                                    )
                                );
                                // put our id back at the begaining and then at the end tha was swap_removeed to the beginig back to the end
                                let last = std::mem::replace(trace.get_mut(0).unwrap(), self.my_id.clone());
                                trace.push(last);

                                let packet = DirectPacket::DistanceIncrease{peer: packet.source, trace};
                                self.direct_broadcast(Some(link_id), packet);
                            }
                        }
                    } else {
                        logy!("networkerror", "we got a trace that didn't start with us");
                    }
                },
/*  Decided to go with a diffrant route :P
                PacketBody::WhoAreYourNeighbors => {
                    let my_neighbors = self.neighbors.keys().map(|x| x.clone()).collect();
                    self.command_queue.push_back(
                        Command::Send { 
                            link_id: link_id.clone(), 
                            packet: Packet{
                                source: self.my_id.clone(), 
                                destination: packet.source, 
                                body: PacketBody::<Answer, Offer>::MyNeighbors(my_neighbors)
                            }
                        }
                    );
                }
*/
            }
        } else {
            self.send_packet(packet);
        }
    }
    pub fn get_next_hop_to(&self, peer_id: &PeerID) -> Option<LinkID> {
        self.neighbors.get(peer_id).cloned()
    }
    pub fn send_direct(&mut self, link_id: LinkID, packet: DirectPacket) {
        self.command_queue.push_back(Command::SendDirect { link_id, packet });
    }
    pub fn send_packet(&mut self, packet: Packet<Answer, Offer>) {
        let link_id = unwrap_or_return!(
            self.get_next_hop_to(&packet.destination),
            logy!("error", "Couldn't find next node to {}", packet.destination),
            ()
        );
        self.command_queue.push_back(Command::Send { link_id, packet });
    }
    pub fn send_overriding_routing(&mut self, link_id: &LinkID, destination: PeerID, packet_body: PacketBody<Answer, Offer>) {
        let packet = Packet { source:self.my_id.clone(), destination, body: packet_body };
        self.command_queue.push_back(Command::Send { link_id: link_id.clone(), packet });
    }
    pub fn send_to(&mut self, destination: PeerID, packet_body: PacketBody<Answer, Offer>) {
        let link_id = unwrap_or_return!(
            self.get_next_hop_to(&destination),
            logy!("error", "Couldn't find next node to {}", destination),
            ()
        );
        let packet = Packet { source:self.my_id.clone(), destination, body: packet_body };
        self.command_queue.push_back(Command::Send { link_id, packet });
    }
    pub fn add_ice(&mut self, link_id: &LinkID, ice: ICE) {
        if let Some(incoming) = self.incoming.get_mut(link_id) {
            incoming.ice.push(ice);
        } else if let Some(outgoing) = self.outgoing.get_mut(link_id) {
            outgoing.ice.push(ice);
        }
    }
}

impl<Answer, Offer> Node<Answer, Offer> {
    pub fn receive_offer(&mut self, offer: Offer, offer_id: OfferID, user: bool) -> LinkID {
        self.link_id_generator_state += 1;
        let link_id: LinkID = self.link_id_generator_state.into();
        self.outgoing.insert(link_id, Outgoing::new(offer_id, None, Vec::new(), user));
        self.command_queue.push_back(Command::GenerateAnswer{link_id: link_id.clone(), offer});
        link_id
    }
    pub fn receive_answer(&mut self, link_id: LinkID, answer: Answer) {
    logy!("trace", "{:?} received answer", self.my_id);
    self.command_queue.push_back(Command::AnswerOffer { link_id, answer });

    }
}

/// Internal
impl<Answer, Offer> Node<Answer, Offer> {
    /// is_keeper() returns  true if we want to keep a connection to the peer and false if we don't mind disconnecting from them
    fn is_keeper(&self, _peer_id: &PeerID) -> bool {
        true
    }
    fn get_peer_id_from_link_id(&self, link_id: &LinkID) -> Option<PeerID> {
        for (peer_id, test_link_id) in &self.neighbors {
            if link_id == test_link_id {
                return Some(peer_id.clone())
            }
        }
        None
    }
}

/// routing
 
impl<Answer: Clone + serde::Serialize, Offer: Clone + serde::Serialize>  Node<Answer, Offer> {
    fn update_routing_table(&mut self, link_id: &LinkID, entries: Vec<(PeerID, RoutingCost)>) {
        for (peer_id, routing_cost) in entries {
            if let Some(current_entry) = self.routing_table.get_mut(&peer_id) {
                if routing_cost <= current_entry.routing_cost {
                    // Update the routing table if a better route is found
                    let _ = std::mem::replace(current_entry, RoutingEntry{next_hop: link_id.clone(), routing_cost: routing_cost + 1});
                } else if link_id == &current_entry.next_hop {
                    self.send_direct(
                        link_id.clone(), 
                        DirectPacket::RouteTraceFromOriginatorToTarget { 
                            target: peer_id, 
                            trace: Vec::from([self.my_id.clone()])
                        }
                    );
                    //overriding_routing(link_id, peer_id.clone(), PacketBody::RequestTraceToMe);
                    //todo!("need to request a trace route to {peer_id}");
                }
            }
        }
    }
    #[allow(dead_code)]
    fn direct_broadcast(&mut self, exclude: Option<&LinkID>, packet:DirectPacket) {
        for (_, neighbor_link) in &self.neighbors {
            if Some(neighbor_link) == exclude {
                continue;
            }
            self.command_queue.push_back(Command::SendDirect { link_id: neighbor_link.clone(), packet: packet.clone() });
        }

    }
}
