use std::{convert::From, fmt, rc::Rc};
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone,Deserialize,Serialize,Hash,PartialEq, Eq, PartialOrd, Ord)]
pub struct PeerID(Rc<str>);
impl From<&str> for PeerID {
    fn from(item: &str) -> Self {
        Self(Rc::from(item))
    }
}
impl From<String> for PeerID {
    fn from(item: String) -> Self {
        Self(Rc::from(item))
    }
}


impl fmt::Display for PeerID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}