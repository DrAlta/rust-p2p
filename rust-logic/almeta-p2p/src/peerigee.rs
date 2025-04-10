use std::collections::HashMap;
use serde::{Deserialize, Serialize};

mod scoring;
use scoring::calculate_score;

use super::PeerID;

pub type Observation = i64;
type PacketID = u128;





#[derive(Debug, Clone,Deserialize,Serialize)]
pub struct Peerigee {
    count_and_first_observed: HashMap<PacketID, (i8, Observation)>,
    observations: HashMap<PeerID, HashMap<PacketID, Observation>>,
    pub keepers: Vec<PeerID>,
}

impl Peerigee {
    fn collect_garbage_after_limit(&mut self, mut limit: Observation) {
        for (_, observations_map) in &mut self.observations {
            observations_map.retain(|_packet_id, observation| observation >= &mut limit);
        }
        self.count_and_first_observed.retain(|_packet_id, (_,observation)| observation >= &mut limit);
    }
    ///  collect_garbage_last_30s(&mut self) just keep the obervations that are in the last 30 obervation of each neighbor
    #[allow(dead_code)]
    fn collect_garbage_last_30s(&mut self) {
        let mut keep = Vec::new();
        for (_, obervations_map) in &mut self.observations {
            let mut x: Vec<(&Observation, &PacketID)> = obervations_map
            .iter()
            .map(
                |(packet_id, observation)|(observation, packet_id)
            )
            .collect();
            x.sort_by(
                |(a_observation, _a_packet_id), (b_observation, _b_packet_id)|
                a_observation.partial_cmp(b_observation ).unwrap_or(std::cmp::Ordering::Equal)
            );
            let mut last_30 = Vec::new();
            for idx in 0..30 {
                let Some((_, &packet_id)) = x.get(idx) else {
                    continue;
                };
                last_30.push(packet_id.clone());
            }
            if obervations_map.len() > 30 {
                obervations_map.retain(|packet_id, _observation| last_30.contains(packet_id));
            }
            keep.append(&mut last_30);
        }
        self.count_and_first_observed.retain(|packet_id, _observation| keep.contains(packet_id));
    }
    pub fn observe(&mut self, peer_id: &PeerID, packet_id: PacketID, observation: Observation ) {
        if let Some((count, previous_observation)) = self.count_and_first_observed.get_mut(&packet_id) {
            let _ = std::mem::replace(count, 1_i8 + *count );
            if &observation < previous_observation {
                let _ = std::mem::replace(previous_observation, observation.clone());
            }
        } else {
            self.count_and_first_observed.insert(
                packet_id.clone(),
                (1, observation.clone())
            );
        }
        if let Some(observation_map) = self.observations.get_mut(peer_id) {
            if !observation_map.contains_key(&packet_id) {
                observation_map.insert(packet_id, observation);
            }
        } else {
            self.observations.insert(peer_id.clone(), HashMap::from([(packet_id, observation)]));
        }
    }

}
//////////////////////////////////////////////////////////

fn collect_observations(observations_map: &HashMap<PacketID, Observation>, count_and_first_observed: &HashMap<PacketID, (i8, Observation)>) -> Vec<Observation>{
    let mut observations = Vec::new();
    for (packet_id, o) in observations_map {
        let Some((times_observed, first_observation)) =  count_and_first_observed.get(packet_id) else {
            continue
        };
        if times_observed >= &3 { 
            observations.push(o - first_observation)
        }
    }
    observations
}

/// public interface
impl Peerigee {
    pub fn new() -> Self {
        Self {
            count_and_first_observed: HashMap::new(),
            observations: HashMap::new(),
            keepers: Vec::new(),
        }
    }
    pub fn is_keeper(&self, peer_id: &PeerID) -> bool {
        self.keepers.contains(peer_id)
    }
    pub fn peerigee(&mut self) -> Option<PeerID> {
        let mut victum = None;
            let mut scores = HashMap::new();
            let mut max_lcb = f64::NEG_INFINITY;
            let mut min_ucb = f64::INFINITY;
    
            for (neighbor_id, observations_map) in &self.observations {
                let observations = collect_observations(observations_map, &self.count_and_first_observed);
   
                let Some([ucb, lcb]) = calculate_score(observations) else {
                    continue
                };
            
                if lcb > max_lcb {
                    max_lcb = lcb;
                    victum = Some(neighbor_id.clone());
                }
                scores.insert(
                    neighbor_id.clone(), 
                    lcb
                );
    
                if ucb < min_ucb {
                    min_ucb = ucb;
                }
            }
            let _= std::mem::replace(&mut self.keepers, get_a_list(scores, crate::node::IDEAL_NUMBER_OF_NEIGHBORS));
    
            if max_lcb > min_ucb {
               return victum; 
            }
            None
        }
    }
pub fn get_a_list(keepers: HashMap<PeerID, f64>, length: usize) -> Vec<PeerID> {
    let mut x: Vec<&PeerID> = keepers.keys().collect();
    x.sort_by(|&a, &b| {
        let left = keepers.get(a).cloned().unwrap();
        let right = keepers.get(b).unwrap();
        left.partial_cmp(right).unwrap_or(std::cmp::Ordering::Equal)
    });
    x.truncate(length);
    x.into_iter().map(|x| x.clone()).collect()
}

//////////////////////////////////////////////////////////
#[allow(dead_code)]
fn main() {
    let mut n = Peerigee::new();
    let a = "a".into();
    let b = "b".into();
    let c = "c".into();
    let d = "d".into();

    n.observe(&a, 1, 10);
    n.observe(&b, 1, 15);
    n.observe(&c, 1, 20);

    n.observe(&a, 2, 20);
    n.observe(&b, 2, 23);
    n.observe(&c, 2, 26);
    n.observe(&d, 2, 30);

    n.observe(&a, 3, 10);
    n.observe(&b, 3, 15);

    n.peerigee();
    println!("{:#?}", n.keepers);
    n.collect_garbage_after_limit(12);
}
