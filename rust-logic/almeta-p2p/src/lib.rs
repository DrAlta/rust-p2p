
mod aux;
use aux::{Incoming, Outgoing};
mod command;
pub use command::Command;
pub mod direct_packet;
pub use direct_packet::DirectPacket;
mod ice;
pub use ice::ICE;
mod node;
pub use node::Node;
pub mod packet;
pub use packet::Packet;
pub mod scoring;
mod user_json;
pub use user_json::UserJSON;

pub type ChannelID = i8;
pub type OfferID = i8;
pub type PeerID = String;


#[macro_export]
macro_rules! logy {
    ($lvl:expr, $($arg:tt)*) => {
        #[cfg(feature = $lvl)]
        println!("[{}:{}:{}]{}", $lvl, file!(), line!(), format!($($arg)*));
    };
}
