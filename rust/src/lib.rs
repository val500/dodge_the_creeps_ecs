use godot::prelude::*;

mod game_scene;
mod ecs;
mod components;
mod resources;
mod player;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
