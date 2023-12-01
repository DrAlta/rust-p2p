
use crate::OfferID;

use super::ICE;

#[derive(Debug)]
pub struct Outgoing<Answer> {
    pub user: bool,
    pub offer_id: OfferID,
    pub answer: Option<Answer>,
    pub ice: Vec<ICE>,
}

impl<Answer> Outgoing<Answer>{
    pub fn new(offer_id: OfferID, answer: Option<Answer>, ice: Vec<ICE>, user: bool) -> Self {
        Self { offer_id, answer, ice, user }
    }
}
#[derive(Debug)]
pub struct Incoming<Offer> {
    pub user: bool,
    pub offer: Option<Offer>,
    pub ice: Vec<ICE>,
}

impl<Offer> Incoming<Offer>{
    pub fn new(offer: Option<Offer>, ice: Vec<ICE>, user: bool) -> Self {
        Self { offer, ice, user}
    }
}
