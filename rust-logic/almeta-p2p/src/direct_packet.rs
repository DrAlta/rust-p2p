use serde::{Deserialize, Serialize};

use super::PeerID;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "Type")]
pub enum DirectPacket {
    Greetings{
        #[serde(rename = "Me")]
        me: PeerID,
        #[serde(rename = "Version")]
        version: String,
    },
    Me{
        #[serde(rename = "Me")]
        me: PeerID,
    },
    Who,
    UnknownVersion,
    NotYouAgain,
    InvalidPacket,
    InvalidSalutation,
    Goodbye,
}

pub fn main(){
    let greetings = DirectPacket::Greetings{me: "Spam".into(), version: "test:0.0".into()};
    println!("\n{}\n", serde_json::to_string(&greetings).unwrap());
}