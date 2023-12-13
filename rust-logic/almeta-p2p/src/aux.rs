use std::{convert::From, fmt};


use serde::{Deserialize, Serialize};

use crate::PeerID;

use super::ICE;

#[derive(Debug)]
pub struct Outgoing<Answer> {
    pub peer: Option<PeerID>,
    pub offer_id: OfferID,
    pub answer: Option<Answer>,
    pub ice: Vec<ICE>,
}

impl<Answer> Outgoing<Answer>{
    pub fn new(offer_id: OfferID, answer: Option<Answer>, ice: Vec<ICE>, peer: Option<PeerID>) -> Self {
        Self { offer_id, answer, ice, peer }
    }
}
#[derive(Debug)]
pub struct Incoming<Offer> {
    pub for_peer: Option<PeerID>,
    pub offer: Option<Offer>,
    pub ice: Vec<ICE>,
}

impl<Offer> Incoming<Offer>{
    pub fn new(offer: Option<Offer>, ice: Vec<ICE>, for_peer: Option<PeerID>) -> Self {
        Self { offer, ice, for_peer}
    }
}

type InnerID = i32;
#[derive(Debug, Clone,Deserialize,Serialize,Hash,PartialEq, Eq, PartialOrd, Ord)]
pub struct OfferID(InnerID);

impl From<i32> for OfferID {
    fn from(item: i32) -> Self {
        OfferID(item)
    }
}
/*impl From<LinkID> for OfferID {
    fn from(item: LinkID) -> Self {
        OfferID(item.to_inner())
    }
}
impl From<&LinkID> for OfferID {
    fn from(item: &LinkID) -> Self {
        OfferID(item.to_inner())
    }
}
*/
impl OfferID {
    pub fn to_inner(&self) -> InnerID {
        self.0
    }
}
impl fmt::Display for OfferID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}


#[derive(Debug, Clone,Deserialize,Serialize,Hash,PartialEq, Eq, PartialOrd, Ord)]
pub struct LinkID(InnerID);
impl fmt::Display for LinkID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl From<i32> for LinkID {
    fn from(item: i32) -> Self {
        LinkID(item)
    }
}
/*
impl From<OfferID> for LinkID {
    fn from(item: OfferID) -> Self {
        LinkID(item.to_inner())
    }
}
*/
impl LinkID {
    pub fn to_inner(&self) -> InnerID {
        self.0
    }
}
