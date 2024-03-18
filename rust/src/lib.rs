use godot::prelude::*;

mod game_scene;
mod components;
mod resources;
mod player;
mod mob;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
