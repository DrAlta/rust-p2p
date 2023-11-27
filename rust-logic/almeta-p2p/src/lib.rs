

mod command;
pub use command::Command;
mod ice;
pub use ice::ICE;
mod node;
pub use node::Node;
pub mod packet;
pub use packet::Packet;


pub type ChannelID = i8;
pub type OfferID = String;
pub type PeerID = String;


#[macro_export]
macro_rules! logy {
    ($lvl:expr, $($arg:tt)*) => {
        #[cfg(feature = $lvl)]
        println!("[{}:{}:{}]{}", $lvl, file!(), line!(), format!($($arg)*));
    };
}
