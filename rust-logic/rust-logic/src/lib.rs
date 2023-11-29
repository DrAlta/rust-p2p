use godot::prelude::*;

mod rust_logic;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
