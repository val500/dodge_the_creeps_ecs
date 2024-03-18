use std::time::Duration;

use godot::engine::AnimatedSprite2D;
use godot::engine::Area2D;
use godot::engine::PathFollow2D;
use godot::prelude::*;
use bevy_ecs::prelude::*;
use crate::resources::*;
use crate::components::*;
use rand::Rng as _;
use std::f32::consts::PI;
use rand::seq::SliceRandom;

#[derive(Component, Default)]
pub struct Mob;

#[derive(Bundle, Default)]
struct MobBundle {
    pub instance_id: GodotInstanceId,
    pub velocity: Velocity,
    pub mob: Mob,
}

pub fn spawn_mobs(mob_scene_id: Res<MobScene>, mut mob_timer: ResMut<MobTimer>, delta: Res<Delta>, game_scene_id: Res<GameSceneInstanceId>, mut commands: Commands) {
    mob_timer.0.tick(Duration::from_secs_f64(delta.0));
    if mob_timer.0.just_finished() {
	let mob_scene: Gd<PackedScene> = Gd::from_instance_id(mob_scene_id.0);
	let mut game_scene: Gd<Node2D> = Gd::from_instance_id(game_scene_id.0);
	
	let mut mob_instance = mob_scene.instantiate_as::<Area2D>();
	let mut mob_spawn_location = game_scene.get_node_as::<PathFollow2D>("MobPath/MobSpawnLocation");
	let mut rng = rand::thread_rng();
        let progress = rng.gen_range(u32::MIN..u32::MAX);
        mob_spawn_location.set_progress(progress as f32);
        mob_instance.set_position(mob_spawn_location.get_position());
	let mut direction = mob_spawn_location.get_rotation() + PI / 2.0;
        direction += rng.gen_range(-PI / 4.0..PI / 4.0);
	mob_instance.set_rotation(direction);
	let range = rng.gen_range(150.0..250.0);
	let velocity = Velocity(Vector2::new(range, 0.0).rotated(real::from_f32(direction)));	
	commands.spawn(MobBundle {
	    instance_id: GodotInstanceId(mob_instance.instance_id()),
	    velocity,
	    ..Default::default()
	});

	let mut sprite = mob_instance.get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");
	sprite.play();
	let anim_names = sprite.get_sprite_frames().unwrap().get_animation_names();
        let anim_names = anim_names.to_vec();
        let mut rng = rand::thread_rng();
        let animation_name = anim_names.choose(&mut rng).unwrap();
        sprite.set_animation(animation_name.into());
	
	game_scene.add_child(mob_instance.upcast());
    }
}

pub fn despawn_mobs(mob_query: Query<(Entity, &GodotInstanceId), With<Mob>>, mut commands: Commands, mut next_state: ResMut<NextState<GameState>>) {
    for (entity, godot_id) in &mob_query {
	commands.entity(entity).despawn();
	let mut godot_entity: Gd<Area2D> = Gd::from_instance_id(godot_id.0);
	godot_entity.free();
    }    
}

pub fn wait_for_start_signal(mut ev_menu: EventReader<InMenuEvent>, mut next_game_state: ResMut<NextState<GameState>>) {
    for ev in ev_menu.read() {
	next_game_state.set(GameState::Menu);
    }	
}
