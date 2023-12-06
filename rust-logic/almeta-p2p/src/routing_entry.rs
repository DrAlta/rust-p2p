use serde::{Deserialize, Serialize};

use super::LinkID;

pub type RoutingCost = u32;

#[derive(Debug, Clone,Deserialize,Serialize,Hash,PartialEq, Eq, PartialOrd, Ord)]
pub struct RoutingEntry {
    pub next_hop: LinkID,
    pub routing_cost: RoutingCost,
}

impl RoutingEntry {
    pub fn new(next_hop: LinkID, routing_cost: RoutingCost) -> Self {
        Self{next_hop, routing_cost}
    }
}
