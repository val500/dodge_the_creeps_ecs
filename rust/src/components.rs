use bevy_ecs::prelude::*;
use godot::prelude::*;

#[derive(Default, Component)]
pub struct Velocity(pub Vector2);

#[derive(Component)]
pub struct GodotInstanceId(pub InstanceId);

impl Default for GodotInstanceId {
    fn default() -> Self { GodotInstanceId(InstanceId::from_i64(1)) }
}
