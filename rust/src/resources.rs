use bevy_ecs::prelude::*;
use godot::prelude::*;

#[derive(Resource, Default)]
pub struct Delta(pub f64);

#[derive(Resource)]
pub struct ScreenSize(pub Vector2);
