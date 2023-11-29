#![allow(dead_code)]
macro_rules! logy {
    ($lvl:expr, $($arg:tt)*) => {
        #[cfg(feature = $lvl)]
        println!("[{}:{}:{}]{}", $lvl, file!(), line!(), format!($($arg)*));
    };
}

mod p2p;

fn main() {
    almeta_p2p::direct_packet::main();
//    p2p::main();
}
