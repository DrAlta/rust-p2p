use std::collections::HashMap;

use super::PeerID;

type Observation = f64;
type PacketID = usize;

struct NeighborInfo {
    first_observed: HashMap<PacketID, Observation>,
    observations: HashMap<PeerID, HashMap<PacketID, Observation>>,
}

impl NeighborInfo {
    fn new() -> Self {
        Self {
            first_observed: HashMap::new(),
            observations: HashMap::new(),
        }
    }
    fn collect_garbage_after_limit(&mut self, mut limit: Observation) {
        for (_, observations_map) in &mut self.observations {
            observations_map.retain(|_packet_id, observation| observation >= &mut limit);
        }
        self.first_observed.retain(|_packet_id, observation| observation >= &mut limit);
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
        self.first_observed.retain(|packet_id, _observation| keep.contains(packet_id));
    }
    pub fn observe(&mut self, peer_id: &PeerID, packet_id: &PacketID, observation: Observation ) {
        if let Some(previous_observation) = self.first_observed.get(packet_id) {
            if &observation < previous_observation {
                self.first_observed.insert(packet_id.clone(), observation.clone());
            }
        }
        if let Some(observation_map) = self.observations.get_mut(peer_id) {
            observation_map.insert(packet_id.clone(), observation);
        } else {
            self.observations.insert(peer_id.clone(), HashMap::from([(packet_id.clone(), observation)]));
        }
    }
    fn ucb_scoring(&self, c: f64) -> Option<PeerID> {
        let mut to_disconnect: Option<PeerID> = None;
        let mut max_lcb = f64::NEG_INFINITY;
        let mut min_ucb = f64::INFINITY;

        for (neighbor_id, observations_map) in &self.observations {
            let observations = collect_all_observations(observations_map, &self.first_observed);
            let [ucb, lcb] = calculate_cb(&observations, c);
           
            if lcb > max_lcb {
                max_lcb = lcb;
                to_disconnect = Some(neighbor_id.clone());
            }

            if ucb < min_ucb {
                min_ucb = ucb;
            }
        }

        if max_lcb > min_ucb {
           return to_disconnect
        } else {
            return None
        }
    }

}
//////////////////////////////////////////////////////////
fn calculate_cb<O: Clone + PartialOrd + std::ops::Add<f64, Output = O> + std::ops::Sub<f64, Output = O>>(observations: &Vec<O>, c: f64) -> [O; 2] {
    let percentile_90 = percentile(observations, 0.9);
    let log_term = f64::ln(observations.len() as f64);

    let ucb = percentile_90.clone() + c * f64::sqrt(log_term / (2.0 * observations.len() as f64));

    let lcb = percentile_90 - c * f64::sqrt(log_term / (2.0 * observations.len() as f64));
    [ucb, lcb]
}

fn percentile<O: Clone + PartialOrd>(observations: &Vec<O>, percentile: f64) -> O {
    let mut sorted_observations = observations.clone();
    sorted_observations.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let index = (percentile * sorted_observations.len() as f64).floor() as usize;
    sorted_observations[index].clone()
}

fn collect_all_observations(observations_map: &HashMap<PacketID, Observation>, first_observed: &HashMap<PacketID, Observation>) -> Vec<Observation>{
    observations_map.into_iter().map(|(packet_id, o)| o - first_observed[packet_id]).collect()
}
//////////////////////////////////////////////////////////
#[allow(dead_code)]
fn main() {
    let c = 0.5;
    let n = NeighborInfo::new();
    let d = n.ucb_scoring(c);
    println!("{d:#?}");
}
