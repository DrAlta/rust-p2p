use almeta_p2p::ChannelID;
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
            node: Node::new(format!("{}", utilities::randi()))
        }
    }
}

#[godot_api]
impl RustLogic {
    #[func]
    fn channel_established(&mut self, channel_id: ChannelID) {
        self.node.channel_established(&channel_id)
    }
    #[func]
    fn generate_offer(&mut self) -> ChannelID {
        self.node.generate_offer(true)
    }
    #[func]
    fn get_answer_json_by_id(&self, offer_id: ChannelID) -> String {
        let Some(thing) = self.node.get_answer_json_by_id(&offer_id) else {
            return "".into()
        };
        thing
    }
    #[func]
    fn get_offer_json_by_id(&self, offer_id: ChannelID) -> String {
        let Some(thing) = self.node.get_offer_json_by_id(&offer_id) else {
            return "".into()
        };
        thing
    }
    

    // signals

    #[func]
    fn on_answer_generated(&mut self, channel_id: ChannelID, answer: Answer) {
        self.node.on_answer_generated(&channel_id, answer)
    }
    #[func]
    fn on_offer_generated(&mut self, channel_id: ChannelID, offer: String) {
        self.node.on_offer_generated(&channel_id, offer)
    }

    #[func]
    fn on_new_ice_candidate(&mut self, channel_id: ChannelID, media: String, index: i64, sdp: String) {
        let ice = ICE::new(media, index, sdp);
        self.node.add_ice(&channel_id, ice);
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
            almeta_p2p::Command::AddICE { channel_id, ice } => {
                dict.insert("Command", "AddICE");
                dict.insert("ChannelID", channel_id);
                dict.insert("Index",ice.index);
                dict.insert("Media",ice.media);
                dict.insert("Name",ice.name);
            },
            almeta_p2p::Command::AnswerOffer { channel_id, answer } => {
                dict.insert("Command", "AnswerOffer");
                dict.insert("ChannelID", channel_id);
                dict.insert("Answer", answer);

            },
            almeta_p2p::Command::GenerateAnswer { channel_id, offer } => {
                dict.insert("Command", "GenerateAnswer");
                dict.insert("ChannelID", channel_id);
                dict.insert("Offer", offer);
            },
            almeta_p2p::Command::GenerateOffer(channel_id) => {
                dict.insert("Command", "GenerateOffer");
                dict.insert("ChannelID", channel_id);

            },
            almeta_p2p::Command::Send { channel_id, packet } => {
                dict.insert("Command", "Send");
                dict.insert("ChannelID", channel_id);
                dict.insert("Packet", serde_json::to_string(&packet).unwrap());

            },
            almeta_p2p::Command::SendDirect { channel_id, packet } => {
                dict.insert("Command", "SendDirect");
                dict.insert("ChannelID", channel_id);
                dict.insert("Packet", serde_json::to_string(&packet).unwrap());
            },
            almeta_p2p::Command::UserAnswer { channel_id, answer } => {
                dict.insert("Command", "UserAnswer");
                dict.insert("ChannelID", channel_id);
                dict.insert("Answer", answer);

            },
            almeta_p2p::Command::UserOffer { channel_id, offer } => {
                dict.insert("Command", "UserOffer");
                dict.insert("ChannelID", channel_id);
                dict.insert("Offer", offer);
            },
        }
        dict
    }


    #[func]
    fn receive_packet(&mut self, channel_id: ChannelID, json_packet: String) {
        if let Ok(packet) = serde_json::from_str::<Packet::<Answer, Offer>>(&json_packet) {
            self.node.receive_packet(&channel_id, packet)
        } else if let Ok(packet) = serde_json::from_str::<DirectPacket>(&json_packet) {
            self.node.receive_direct(&channel_id, packet)
        } else {
            self.node.command_queue.push_back(
                almeta_p2p::Command::SendDirect { 
                    channel_id, 
                    packet: DirectPacket::InvalidPacket
                }
            )
        };
    }
}
