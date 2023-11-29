use godot::prelude::*;
use godot::engine::RefCounted;
use godot::engine::IRefCounted;

use almeta_p2p::Node;

#[derive(GodotClass)]
#[class(base=RefCounted)]
struct RustLogic {
    #[base]
    base: Base<RefCounted>,

    node: Node<String, String>,
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
    fn foo(){
        //
    }
}
