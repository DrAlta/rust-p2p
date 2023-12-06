
mod aux;
use aux::{Incoming, Outgoing};
pub mod babel;
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
mod peer_id;
pub use peer_id::PeerID;
pub mod routing_entry;
pub mod scoring;
mod user_json;
pub use user_json::UserJSON;

pub type LinkID = i8;
pub type OfferID = i8;


#[macro_export]
macro_rules! logy {
    ($lvl:expr, $($arg:tt)*) => {
        #[cfg(feature = $lvl)]
        println!("[{}:{}:{}]{}", $lvl, file!(), line!(), format!($($arg)*));
    };
}
