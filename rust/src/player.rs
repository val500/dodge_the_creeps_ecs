use bevy_ecs::prelude::*;
use godot::engine::{Area2D, AnimatedSprite2D};
use crate::components::{GodotInstanceId, Velocity};
use crate::resources::{Delta, ScreenSize};
use godot::prelude::*;

#[derive(Component, Default)]
pub struct Player;

#[derive(Component, Default)]
pub enum PlayerAnimationState {
    RIGHT,
    LEFT,
    UP,
    DOWN,
    #[default]
    STOPPED
}

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    pub instance_id: GodotInstanceId,
    pub velocity: Velocity,
    pub animation_state: PlayerAnimationState,
    pub player: Player,
}

#[derive(Resource, Default)]
pub struct PlayerSpeed { pub value: real }

pub fn process_player_input(mut query: Query<(&mut Velocity, &mut PlayerAnimationState), With<Player>>, speed: Res<PlayerSpeed>) {
    let input = Input::singleton();    
    let (mut velocity, mut animation_state) = query.single_mut();
    let mut new_velo_vector = Vector2::ZERO;
    if input.is_action_pressed("move_right".into()) {
	new_velo_vector += Vector2::RIGHT;
	*animation_state = PlayerAnimationState::RIGHT;
    }
    if input.is_action_pressed("move_left".into()) {
	new_velo_vector += Vector2::LEFT;
	*animation_state = PlayerAnimationState::LEFT;
    }
    if input.is_action_pressed("move_up".into()) {
	new_velo_vector += Vector2::UP;
	*animation_state = PlayerAnimationState::UP;
    }
    if input.is_action_pressed("move_down".into()) {
	new_velo_vector += Vector2::DOWN;
	*animation_state = PlayerAnimationState::DOWN;
    }
    if new_velo_vector.length() <= 0.0 {
	*animation_state = PlayerAnimationState::STOPPED;
    }
    new_velo_vector = new_velo_vector.normalized() * speed.value;
    velocity.0 = new_velo_vector;
}

pub fn move_entities(mut query: Query<(&GodotInstanceId, &Velocity)>, delta: Res<Delta>, screen_size: Res<ScreenSize>) {
    for (godot_instance_id, velocity) in query.iter() {	
	let mut godot_entity: Gd<Area2D> = Gd::from_instance_id(godot_instance_id.0);
	let change = velocity.0 * real::from_f64(delta.0);
	let position = godot_entity.get_global_position() + change;
	let position = Vector2::new(
            position.x.clamp(0.0, screen_size.0.x),
            position.y.clamp(0.0, screen_size.0.y),
        );
	godot_entity.set_global_position(position);
    }
}

pub fn animate_player(mut query: Query<(&GodotInstanceId, &PlayerAnimationState)>) {
    for (godot_instance_id, animation_state) in query.iter() {
	let godot_entity: Gd<Area2D> = Gd::from_instance_id(godot_instance_id.0);
	let mut animated_sprite = godot_entity.get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");
	animated_sprite.set_flip_v(false);
	animated_sprite.set_flip_h(false);
	match animation_state {
	    PlayerAnimationState::RIGHT => {
		animated_sprite.set_animation("walk".into());
		animated_sprite.play();
	    }
	    PlayerAnimationState::LEFT => {
		animated_sprite.set_flip_h(true);
		animated_sprite.set_animation("walk".into());
		animated_sprite.play()
	    }
	    PlayerAnimationState::UP => {
		animated_sprite.set_animation("up".into());
		animated_sprite.play();
	    }
	    PlayerAnimationState::DOWN => {
		animated_sprite.set_flip_v(true);
		animated_sprite.set_animation("up".into());
		animated_sprite.play();
	    }
	    PlayerAnimationState::STOPPED => {
		animated_sprite.stop();
	    }
	};
	
    }
}
