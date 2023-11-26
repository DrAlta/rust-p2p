 

pub fn main(){
    let inner = PacketType::Answer { answer: "spam".into(), offer_id: "eggs".into(), ice: Vec::from([ICE::new("ham".into())]) };
    //let outer = Outer::Answer(inner);
    let packet = Packet{source: "Source".into(), destination: "Destination".into(),r#type: inner};
    println!("{}", serde_json::to_string(&packet).unwrap());

}