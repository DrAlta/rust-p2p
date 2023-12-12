type LinkID = i32;
use almeta_p2p::ICE;
use godot::prelude::*;
use godot::engine::RefCounted;
use godot::engine::IRefCounted;

use almeta_p2p::{DirectPacket, Node, Packet};

type Answer = String;
type Offer = String;

#[derive(GodotClass)]
#[class(base=RefCounted)]
struct RustLogic {
    #[base]
    base: Base<RefCounted>,

    node: Node<Answer, Offer>,
}


#[godot_api]
impl IRefCounted for RustLogic {
    fn init(base: Base<RefCounted>) -> Self {
        godot_print!("Hello, world!"); // Prints to the Godot console
        
        Self {
            base,
            node: Node::new(format!("{}", utilities::randi()), utilities::randi() as u64 )
        }
    }
}

#[godot_api]
impl RustLogic {
    #[func]
    pub fn channel_closed(&mut self, link_id: LinkID) {
        self.node.link_closed(&link_id.into())
    }
    #[func]
    fn channel_established(&mut self, link_id: LinkID) {
        self.node.link_established(&link_id.into())
    }
    #[func]
    fn generate_offer(&mut self) -> LinkID {
        self.node.generate_offer(None).0.to_inner()
    }
    #[func]
    fn get_answer_json_by_id(&self, offer_id: LinkID) -> String {
        let Some(thing) = self.node.get_answer_json_by_id(&offer_id.into()) else {
            return "".into()
        };
        thing
    }
    #[func]
    fn get_offer_json_by_id(&self, offer_id: LinkID) -> String {
        let Some(thing) = self.node.get_offer_json_by_id(&offer_id.into()) else {
            return "".into()
        };
        thing
    }
    

    // signals

    #[func]
    fn on_answer_generated(&mut self, link_id: LinkID, answer: Answer) {
        self.node.on_answer_generated(&link_id.into(), answer)
    }
    #[func]
    fn on_offer_generated(&mut self, link_id: LinkID, offer: String) {
        self.node.on_offer_generated(&link_id.into(), offer)
    }

    #[func]
    fn on_new_ice_candidate(&mut self, link_id: LinkID, media: String, index: i64, sdp: String) {
        let ice = ICE::new(media, index, sdp);
        self.node.add_ice(&link_id.into(), ice);
    }


    // binding


    #[func]
    fn poll(&mut self) -> Dictionary {

        let mut dict = Dictionary::new();
        let Some(command) = self.node.command_queue.pop_front() else {
            dict.insert("Command", "Null");
            return dict;
        };
        match command {
            almeta_p2p::Command::AddICE { link_id, ice } => {
                dict.insert("Command", "AddICE");
                dict.insert("LinkID", link_id.to_inner());
                dict.insert("Index",ice.index);
                dict.insert("Media",ice.media);
                dict.insert("Name",ice.name);
            },
            almeta_p2p::Command::AnswerOffer { link_id, answer } => {
                dict.insert("Command", "AnswerOffer");
                dict.insert("LinkID", link_id.to_inner());
                dict.insert("Answer", answer);

            },
            almeta_p2p::Command::GenerateAnswer { link_id, offer } => {
                dict.insert("Command", "GenerateAnswer");
                dict.insert("LinkID", link_id.to_inner());
                dict.insert("Offer", offer);
            },
            almeta_p2p::Command::GenerateOffer(link_id) => {
                dict.insert("Command", "GenerateOffer");
                dict.insert("LinkID", link_id.to_inner());

            },
            almeta_p2p::Command::Send { link_id, packet } => {
                dict.insert("Command", "Send");
                dict.insert("LinkID", link_id.to_inner());
                dict.insert("Packet", serde_json::to_string(&packet).unwrap());

            },
            almeta_p2p::Command::SendDirect { link_id, packet } => {
                dict.insert("Command", "SendDirect");
                dict.insert("LinkID", link_id.to_inner());
                dict.insert("Packet", serde_json::to_string(&packet).unwrap());
            },
            almeta_p2p::Command::UserAnswer { link_id, answer } => {
                dict.insert("Command", "UserAnswer");
                dict.insert("LinkID", link_id.to_inner());
                dict.insert("Answer", answer);

            },
            almeta_p2p::Command::UserOffer { link_id, offer } => {
                dict.insert("Command", "UserOffer");
                dict.insert("LinkID", link_id.to_inner());
                dict.insert("Offer", offer);
            },
        }
        dict
    }


    #[func]
    fn receive_packet(&mut self, link_id: LinkID, json_packet: String) {
        if let Ok(packet) = serde_json::from_str::<Packet::<Answer, Offer>>(&json_packet) {
            self.node.receive_packet(&link_id.into(), packet)
        } else if let Ok(packet) = serde_json::from_str::<DirectPacket>(&json_packet) {
            self.node.receive_direct(&link_id.into(), packet)
        } else {
            self.node.command_queue.push_back(
                almeta_p2p::Command::SendDirect { 
                    link_id: link_id.into(), 
                    packet: DirectPacket::InvalidPacket
                }
            )
        };
    }
    /* this is how to conver a sting into a PackedByteArray
    #[func]
    pub fn test() -> godot::builtin::PackedByteArray {
        "test".to_string().as_bytes().into()
    }
    */

}
