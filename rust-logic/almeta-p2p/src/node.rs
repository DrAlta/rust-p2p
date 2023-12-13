use std::collections::{HashMap, VecDeque};
use qol::{logy, unwrap_or_return};
use crate::{OfferID, packet::PacketBody};

use super::{LinkID, Command, ICE, direct_packet::DirectPacket, DirectBody, Packet, PeerID, Perigee, routing_entry::{RoutingCost, RoutingEntry}, Incoming, Outgoing};

const PROTOCAL_VERSION: &str = concat!("Rust", ":", "0.1");
const IDEAL_NUMBER_OF_NEIGHBORS: usize = 8;
#[derive(Debug)]
struct LinkInfo {
    pub protocal_in: Option<String>,
    pub protocal_out: Option<String>,
    pub post_init_greeting: bool,
}

impl LinkInfo{
    pub fn new() -> Self {
        Self { protocal_in: None, protocal_out: None, post_init_greeting: false }
    }
}
#[derive(Debug)]
pub struct Node<Answer, Offer> {
    rng: thats_so_random::Pcg32,
    pub command_queue: VecDeque<Command<Answer, Offer>>,

    pub my_id: PeerID,

    neighbors: HashMap<PeerID, LinkID>,

    routing_table: HashMap<PeerID, RoutingEntry>,
    
    link_id_generator_state: i32,

    incoming: HashMap<OfferID, Incoming<Offer>>,
    outgoing: HashMap<LinkID, Outgoing<Answer>>,

    link_info: HashMap<LinkID, LinkInfo>,

    perigee: Perigee
}

/// Creation
impl<Answer, Offer> Node<Answer, Offer> {
    pub fn new(my_id: String, state: u64) -> Self {
        let my_id = my_id.into();
        Self {
            rng: thats_so_random::Pcg32::new(state, thats_so_random::DEFAULT_STREAM),
            command_queue: VecDeque::new(), 
            my_id, 
            neighbors: HashMap::new(),
            routing_table: HashMap::new(),
            link_id_generator_state: 0, 
            incoming: HashMap::new(),
            outgoing: HashMap::new(),
            link_info: HashMap::new(),

            perigee: Perigee::new(),
        }
    }
}

impl<Answer: Clone + std::fmt::Debug + serde::Serialize, Offer: Clone + std::fmt::Debug + serde::Serialize> Node<Answer, Offer> {
    /*
    pub fn generate_offer(&mut self) -> LinkID {
        self.link_id_generator_state += 1;
        let link_id: LinkID = self.link_id_generator_state.into();
        self.command_queue.borrow_mut().push(Command::GenerateOffer(link_id.clone()));
        link_id
    }
    */
    pub fn link_closed(&mut self, link_id: &LinkID) {
        self.neighbors.retain(|_peer_id, test_link_id| test_link_id != link_id);
        self.incoming.remove(&self.link_id_to_offer_id(link_id));
        self.outgoing.remove(link_id);
        self.link_info.remove(link_id);
        self.eval_neighbors();
    }
    pub fn link_established(&mut self, link_id: &LinkID) {
        logy!("tracenode", "connection established on link:{link_id}");
        self.incoming.remove(&self.link_id_to_offer_id(link_id));
        self.outgoing.remove(link_id);

        self.link_info.insert(link_id.clone(), LinkInfo::new());
  
        self.send_direct(link_id.clone(), DirectBody::Greetings { me: self.my_id.clone(), supported_versions: Vec::from([PROTOCAL_VERSION.into()]) }.into())
    }
    pub fn get_answer_json_by_id(&self, link_id: &LinkID) -> Option<String> {
        let outgoing = self.outgoing.get(link_id)?;
        let answer = outgoing.answer.clone()?;
        let inner = PacketBody::<Answer, Offer>::Answer { 
            answer: answer, 
            offer_id: outgoing.offer_id.clone(), 
            ice: outgoing.ice.clone()
        };
        let user = Packet::new(
            "User".into(),
            self.my_id.clone(),
            inner
        );
        serde_json::to_string(&user).ok()
    }
    pub fn get_neighbors(&self) -> Vec<PeerID> {
        self.neighbors.keys().map(|x| x.clone()).collect()
    }
    pub fn get_offer_json_by_id(&self, offer_id: &OfferID) -> Option<String> {
        let incoming = self.incoming.get(offer_id)?;
        let offer = incoming.offer.clone()?;
        let inner = PacketBody::<Answer, Offer>::Offer { 
            offer: offer, 
            offer_id: offer_id.clone(),
            ice: incoming.ice.clone()
        };
        let user = Packet::new(
            "User".into(),
            self.my_id.clone(),
            inner
        );
        serde_json::to_string(&user).ok()
    }
    pub fn generate_offer(&mut self, for_peer: Option<PeerID>) -> (LinkID, OfferID) {
        self.link_id_generator_state += 1;
        let link_id: LinkID = self.link_id_generator_state.into();
        let offer_id: OfferID = self.link_id_generator_state.into();
        self.incoming.insert(offer_id.clone(), Incoming::new(None, Vec::new(), for_peer));
        self.command_queue.push_back(Command::GenerateOffer(link_id.clone()));
        (link_id, offer_id)
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
            logy!("error", "coundn't find outgoing {link_id}"),
            ()
        );
        outgoing.answer = Some(answer.clone());
        let Some(peer_id) = outgoing.peer.clone() else {
            self.command_queue.push_back(Command::UserAnswer { link_id: link_id.clone(), answer: answer });
            return
        };
        let packet = PacketBody::Answer { answer, offer_id: outgoing.offer_id.clone(), ice: outgoing.ice.clone() };
        self.send_to(peer_id, packet)
    }
    // this should take a link_id on the other side of the binding is only aware on links and not the interstuff like Offers
    pub fn on_offer_generated(&mut self, link_id: &LinkID, offer: Offer) {
        let offer_id = self.link_id_to_offer_id(link_id);
        let incoming = unwrap_or_return!(
            self.incoming.get_mut(&offer_id),
            logy!("error", "incoming {link_id:?} not found"),
            ()   
        );
        incoming.offer = Some(offer.clone());
        if let Some(peer_id) = incoming.for_peer.clone() {
            logy!("tracenode", " received offer for {peer_id:?} on channel {link_id:?}");
            let packet_body = PacketBody::Offer { offer, offer_id, ice: incoming.ice.clone()};
            self.send_to(peer_id.clone(), packet_body);
        } else {
            logy!("tracenode", " received offer for user on channel {link_id:?}");
            self.command_queue.push_back(Command::UserOffer { link_id: link_id.clone(), offer: offer.clone() });
        }
    }
    pub fn receive_answer(&mut self, link_id: LinkID, answer: Answer) {
        logy!("trace", "{:?} received answer", self.my_id);
        self.command_queue.push_back(Command::AnswerOffer { link_id, answer });
    }
    pub fn receive_direct(&mut self, link_id: &LinkID, packet: DirectPacket) {
        if let Some(link_info) = self.link_info.get(link_id) {
            match link_info.protocal_in {
                Some(_) => {
                    self.receive_direct_rust0_1(link_id, packet);
                },
                None =>{
                    self.receive_direct_rust0_1(link_id, packet);
                }
            }
        } else {
            logy!("error", "couldn't find link_info for {link_id}");
        }

        if let Some(link_info) = self.link_info.get_mut(link_id) {
            match (&link_info.protocal_out, &link_info.post_init_greeting) {
                (Some(_), false) => {
                    logy!("debug", "should be sending post greating");
                    link_info.post_init_greeting = true;
                    self.post_greeting(link_id);    
                },
                _ => (),

            }
        } else {
            logy!("debug", "Cuoldn't get mut");

        }

    }
    pub fn receive_direct_rust0_1(&mut self, link_id: &LinkID, packet: DirectPacket) {
        logy!("traenode", "processing direct rust:0.1 packet");
        if packet.varify() {
            match packet.body {
                DirectBody::DearJohn => {
                    let peer_id = unwrap_or_return!( self.get_peer_id_from_link_id(link_id));
                    if ! self.is_keeper(&peer_id) {
                        self.send_direct(link_id.clone(), DirectBody::Goodbye.into());
                    }
                },
                DirectBody::DistanceIncrease { peer, mut trace } => {
                    // we don't keep routing info about ourself.
                    if &peer == &self.my_id {
                        return;
                    }
                    let next_hop = unwrap_or_return!(self.get_next_hop_to(&peer));
                    if link_id == &next_hop {
                        // check if we in the trace;
                        if trace.contains(&self.my_id) {
                            let packet= DirectBody::LostRouteTo{peer}.into();
                            self.direct_broadcast(None, packet);
                            return
                        }
                        trace.push(self.my_id.clone());
                        logy!("tracenode", "{} adding {} to RT", self.my_id, peer);
                        self.routing_table.insert(peer.clone(), RoutingEntry::new(next_hop, trace.len() as u32));
                        let packet= DirectBody::DistanceIncrease { peer, trace}.into();
                        self.direct_broadcast(Some(link_id), packet);
                    }
                    
                },
                DirectBody::Goodbye => todo!(),
                DirectBody::Greetings { me, supported_versions } => {
                    let version = PROTOCAL_VERSION.into();
                    if supported_versions.contains(&version) {
                        if self.neighbors.get(&me).is_some() {
                            logy!("tracenode", "Greetings {me} was in neighbors");
                            self.send_direct(link_id.clone(), DirectBody::NotYouAgain.into())
                        } else {
                            if let Some(link_info) = self.link_info.get_mut(link_id) {
                                link_info.protocal_out = Some(version.clone()); 
                            }
                            logy!("tracenode", "adding {me} to neighbors");
                            self.neighbors.insert(me.clone(), link_id.clone());
                            logy!("tracenode", "{} adding {} to RT", self.my_id, me);
                            self.routing_table.insert(
                                me, 
                                RoutingEntry::new(
                                    link_id.clone(), 
                                    0
                                )
                            );

                            self.send_direct(link_id.clone(), DirectBody::TellItToMeIn{version}.into());
                        }
                    } else {
                        self.send_direct(link_id.clone(), DirectBody::UnknownVersion.into());
                    }
                },
                DirectBody::InvalidPacket => todo!(),
                DirectBody::InvalidSalutation => todo!(),
                DirectBody::LostRouteTo{peer} => {
                    let RoutingEntry { next_hop, routing_cost } = unwrap_or_return!(self.routing_table.get(&peer));
                    if next_hop == link_id {
                        let packet= DirectBody::LostRouteTo{peer}.into();
                        self.direct_broadcast(None, packet);
                    } else {
                        let packet = DirectBody::RoutingInformationExchange{
                            entries: Vec::from(
                                [
                                    (peer, routing_cost.clone())
                                ]
                            )
                        }.into();
                        self.send_direct(link_id.clone(), packet);
                    }
                },
                DirectBody::Me { .. } => {

                },
                DirectBody::NotYouAgain => {
                    // ToDo
                    ()
                },
                DirectBody::RouteTraceFromOriginatorToTarget { target, mut trace } => {
                    let originator = unwrap_or_return!(trace.get(0));
                    if &self.my_id == &target {
                        let target = originator.clone();
                        let packet_body = PacketBody::ReturnRouteTrace{trace};
                        self.send_to(target, packet_body);
                        return
                    }
                    let next_hop = unwrap_or_return!(self.get_next_hop_to(&target));
                    trace.push(self.my_id.clone());
                    let new_packet = DirectBody::RouteTraceFromOriginatorToTarget{
                        target, 
                        trace
                    }.into();
                    self.send_direct(next_hop, new_packet)
                },
                DirectBody::RouteTraceToOriginatorFromTarget { originator, mut trace } => {
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
                        let new_packet = DirectBody::RouteTraceToOriginatorFromTarget{
                            originator, 
                            trace
                        }.into();
                        self.send_direct(next_hop, new_packet)
                    }
                },

                DirectBody::RoutingInformationExchange{entries} => {
                    self.update_routing_table(link_id, entries);
                },
                DirectBody::TellItToMeIn{version} => {
                    if let Some(link_info) = self.link_info.get_mut(link_id) {
                        link_info.protocal_out = Some(version.clone()); 
                        
                    }
                }
                DirectBody::UnknownVersion => todo!(),
                DirectBody::Who => {
                    self.send_direct(link_id.clone(), DirectBody::Me { me: self.my_id.clone() }.into());
                },
            };
        }
    }
    pub fn receive_offer(&mut self, offer: Offer, offer_id: OfferID, peer: Option<PeerID>) -> LinkID {
        self.link_id_generator_state += 1;
        let link_id: LinkID = self.link_id_generator_state.into();
        self.outgoing.insert(link_id.clone(), Outgoing::new(offer_id, None, Vec::new(), peer));
        self.command_queue.push_back(Command::GenerateAnswer{link_id: link_id.clone(), offer});
        logy!("tracenode", "receive_offer  linkID:{}", link_id);
        link_id
    }
    pub fn receive_packet(&mut self, link_id: &LinkID, packet: Packet<Answer, Offer>) {
        if let Some(link_info) = self.link_info.get(link_id) {
            match link_info.protocal_in.as_ref() {
                _ => {
                    self.receive_packet_rust0_1(link_id, packet)
                }
            }
        }
    }    
    pub fn receive_packet_rust0_1(&mut self, link_id: &LinkID, packet: Packet<Answer, Offer>) {
        if packet.destination == self.my_id {
            match packet.body {
                PacketBody::Answer { answer, offer_id, ice} => {
                    let answer_link_id = self.offer_id_to_link_id(&offer_id);
                    self.command_queue.push_back(Command::AnswerOffer { link_id: answer_link_id.clone(), answer });
                    for icee in ice {
                        self.command_queue.push_back(Command::AddICE { link_id: answer_link_id.clone(), ice: icee});
                    };
                },
                PacketBody::Goodbye => todo!(),
                PacketBody::InvalidPacket => {
                    logy!("error", "Got a reply that I sent a invalid packet");
                },
                PacketBody::Offer {offer, offer_id, ice} => {
                    let id = self.receive_offer(offer, offer_id, None);
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
                        self.incoming.get_mut(&self.link_id_to_offer_id(&link_id)),
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
                PacketBody::RequestOffer => {
                    self.generate_offer(Some(packet.source));
                },
                PacketBody::RequestTraceToMe => {
                    let next_hop =  unwrap_or_return!(self.get_next_hop_to(&packet.source));

                    let new_packet = DirectBody::RouteTraceToOriginatorFromTarget{
                        originator: packet.source, 
                        trace: Vec::from([self.my_id.clone()])
                    }.into();
                    self.send_direct(next_hop, new_packet)
                },
                PacketBody::ReturnRouteTrace{mut trace} => {
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
                                    let packet= DirectBody::LostRouteTo{peer: packet.source.clone()}.into();
                                    self.direct_broadcast(None, packet);
                                    return
                                }
                                logy!("tracenode", "{} adding {} to RT", self.my_id, packet.source);
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

                                let packet = DirectBody::DistanceIncrease{peer: packet.source, trace}.into();
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
        if peer_id == "User" {
            return Some(0.into())
        }
        if let Some(neighbor) = self.neighbors.get(peer_id).cloned() {
            return Some(neighbor)
        } else {
            let Some(x) = self.routing_table.get(peer_id) else {
                return None
            };
            return Some(x.next_hop.clone())
        }
    }
    pub fn send_direct(&mut self, link_id: LinkID, packet: DirectPacket) {
        Self::send_direct_inner(&mut self.command_queue, link_id, packet);
    }
    pub fn send_direct_inner(command_queue: &mut VecDeque<Command<Answer, Offer>>, link_id: LinkID, packet: DirectPacket) {
        command_queue.push_back(Command::SendDirect { link_id, packet });
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
        let packet = Packet::new( self.my_id.clone(), destination, packet_body );
        self.command_queue.push_back(Command::Send { link_id: link_id.clone(), packet });
    }
    pub fn send_to(&mut self, destination: PeerID, packet_body: PacketBody<Answer, Offer>) {
        let link_id = unwrap_or_return!(
            self.get_next_hop_to(&destination),
            logy!("error", "Couldn't find next node to {}", destination),
            ()
        );
        let packet = Packet::new(self.my_id.clone(), destination, packet_body );
        self.command_queue.push_back(Command::Send { link_id, packet });
    }
    pub fn add_ice(&mut self, link_id: &LinkID, ice: ICE) {
        if let Some(incoming) = self.incoming.get_mut(&self.link_id_to_offer_id(link_id)) {
            incoming.ice.push(ice);
        } else if let Some(outgoing) = self.outgoing.get_mut(link_id) {
            outgoing.ice.push(ice);
        }
    }
}


/// Internal
impl<Answer, Offer> Node<Answer, Offer> {
    /// is_keeper() returns  true if we want to keep a connection to the peer and false if we don't mind disconnecting from them
    fn is_keeper(&self, peer_id: &PeerID) -> bool {
        if self.perigee.keepers.len() < IDEAL_NUMBER_OF_NEIGHBORS {
            return true
        }
        self.perigee.is_keeper(peer_id)
    }
    fn get_peer_id_from_link_id(&self, link_id: &LinkID) -> Option<PeerID> {
        for (peer_id, test_link_id) in &self.neighbors {
            if link_id == test_link_id {
                return Some(peer_id.clone())
            }
        }
        None
    }
   fn link_id_to_offer_id(&self, link_id: &LinkID) -> OfferID {
        link_id.to_inner().into()
    }
    fn offer_id_to_link_id(&self, offer_id: &OfferID) -> LinkID {
        offer_id.to_inner().into()
    }
}

/// routing
 
impl<Answer: Clone + std::fmt::Debug + serde::Serialize, Offer: Clone + std::fmt::Debug + serde::Serialize>  Node<Answer, Offer> {
    fn update_routing_table(&mut self, link_id: &LinkID, entries: Vec<(PeerID, RoutingCost)>) {
        println!(">>>[node:{}]rt:{:?}\n>>>[node:{}]new{:?}", line!(), self.routing_table, line!(), entries);
        for (peer_id, routing_cost) in entries {
            // we don't keep routing info about ourself.
            if &peer_id == &self.my_id {
                continue;
            }
            if let Some(current_entry) = self.routing_table.get_mut(&peer_id) {
                if routing_cost <= current_entry.routing_cost {
                    // Update the routing table if a better route is found
                    let _ = std::mem::replace(current_entry, RoutingEntry{next_hop: link_id.clone(), routing_cost: routing_cost + 1});
                } else if link_id == &current_entry.next_hop {
                    self.send_direct(
                        link_id.clone(), 
                        DirectBody::RouteTraceFromOriginatorToTarget { 
                            target: peer_id, 
                            trace: Vec::from([self.my_id.clone()])
                        }.into()
                    );
                    //overriding_routing(link_id, peer_id.clone(), PacketBody::RequestTraceToMe);
                    //todo!("need to request a trace route to {peer_id}");
                }
            } else {
                logy!("tracenode", "{} adding {} to RT", self.my_id, peer_id);
                self.routing_table.insert(peer_id, RoutingEntry{next_hop: link_id.clone(), routing_cost: routing_cost + 1});
            }
        }
        self.eval_neighbors()
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
    fn post_greeting(&mut self, link_id: &LinkID) {
        let routing_info: Vec::<(PeerID, RoutingCost)>;
        if let Some(them) = self.get_peer_id_from_link_id(link_id){ 
            routing_info= (&self.routing_table).into_iter().filter(|(x, _)|x == &&them).map(|(peer_id, routing_entry)| (peer_id.clone(), routing_entry.routing_cost)).collect();
        } else {
            routing_info= (&self.routing_table).into_iter().map(|(peer_id, routing_entry)| (peer_id.clone(), routing_entry.routing_cost)).collect();
        }
        if !routing_info.is_empty() {
            self.send_direct(link_id.clone(), DirectBody::RoutingInformationExchange{entries: routing_info}.into());
        }
    }
}

impl<Answer: Clone + std::fmt::Debug + serde::Serialize, Offer: Clone + std::fmt::Debug + serde::Serialize> Node<Answer, Offer> {
    pub fn eval_neighbors(&mut self) {
        self.perigee.perigee(0.5);
        for (peer_id, link_id) in &self.neighbors {
             if ! self.is_keeper(peer_id) {
                Self::send_direct_inner(&mut self.command_queue, link_id.clone(), DirectBody::DearJohn.into());
             }
        }
        let number_of_neighbors = self.neighbors.len();
        if number_of_neighbors > IDEAL_NUMBER_OF_NEIGHBORS {
            logy!("tracenode", "{} has enough naighbors", self.my_id);
            return;
        }
        let mut available: Vec<PeerID> = self.routing_table.keys().filter(|&x| !self.neighbors.contains_key(x)).map(|x| x.clone()).collect();
        logy!("tracenode", "{} has availible:{available:?}", self.my_id);
        for idx in 0..(IDEAL_NUMBER_OF_NEIGHBORS - number_of_neighbors).max(0) {
            if available.is_empty() {
                logy!("tracenode", "{} exhausted available peers after {idx}", self.my_id);
                break;
            }
            let peer_id = self.rng.random_item(&mut available).expect("we already checked that available wasn't empty so there is no reason we shoulfn't be able to get an item from it");
            self.send_to(peer_id.clone(), PacketBody::RequestOffer);            
        }
    }
    pub fn share_routing_info(&mut self) {
        let routing_info: Vec<(PeerID, u32)>= (&self.routing_table).into_iter().map(|(peer_id, routing_entry)| (peer_id.clone(), routing_entry.routing_cost)).collect();
        for (neighbor_peer_id, neighbot_link_id) in self.neighbors.clone().into_iter(){
            let entries = routing_info.clone().into_iter().filter(|(x, _)| x != &neighbor_peer_id).collect();
            self.send_direct(neighbot_link_id, DirectBody::RoutingInformationExchange{entries}.into());
        }
    }
    pub fn tick(&mut self) {
        self.eval_neighbors();
        self.share_routing_info();
    }
 
 }