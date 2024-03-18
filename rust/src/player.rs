use std::time::Duration;

use bevy_ecs::prelude::*;
use godot::engine::{Area2D, AnimatedSprite2D, Marker2D, CanvasLayer};
use crate::components::{GodotInstanceId, Velocity};
use crate::resources::*;
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

pub fn move_entities(query: Query<(Entity, &GodotInstanceId, &Velocity, Option<&Player>)>, delta: Res<Delta>, screen_size: Res<ScreenSize>, mut commands: Commands) {
    for (entity, godot_instance_id, velocity, player) in query.iter() {	
	let mut godot_entity: Gd<Area2D> = Gd::from_instance_id(godot_instance_id.0);
	let change = velocity.0 * real::from_f64(delta.0);
	let mut position = godot_entity.get_global_position() + change;
	if let Some(_player) = player {
	    position = Vector2::new(
		position.x.clamp(0.0, screen_size.0.x),
		position.y.clamp(0.0, screen_size.0.y),
            );
	} else {
	    if !(0.0..screen_size.0.x).contains(&position.x) ||
		!(0.0..screen_size.0.x).contains(&position.x) {
		    commands.entity(entity).despawn();
		    godot_entity.free();
		    godot_print!("despawn");			
		    return;
	    }
	}
	godot_entity.set_global_position(position);
    }
}

pub fn animate_player(query: Query<(&GodotInstanceId, &PlayerAnimationState)>) {
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

pub fn detect_player_collisions(query: Query<(Entity, &GodotInstanceId), With<Player>>, mut commands: Commands, mut next_state: ResMut<NextState<GameState>>, game_scene_id: ResMut<GameSceneInstanceId>) {
    let (entity_id, godot_instance_id) = query.single();
    let mut godot_entity: Gd<Area2D> = Gd::from_instance_id(godot_instance_id.0);
    if godot_entity.has_overlapping_areas() {
	godot_entity.hide();
	let game_scene: Gd<Node2D> = Gd::from_instance_id(game_scene_id.0);
	let mut hud = game_scene.get_node_as::<CanvasLayer>("HUD");
	hud.emit_signal("game_over_signal".into(), &[]);
	next_state.set(GameState::GameOver);
	
    }
}

pub fn start_game(query: Query<(&GodotInstanceId), With<Player>>, mut ev_start_game: EventReader<StartGameEvent>, game_scene_id: Res<GameSceneInstanceId>, mut next_game_state: ResMut<NextState<GameState>>, mut score: ResMut<Score>, mut score_timer: ResMut<ScoreTimer>) {
    for ev in ev_start_game.read() {
	let godot_instance_id = query.single();
	let mut player_entity: Gd<Area2D> = Gd::from_instance_id(godot_instance_id.0);
	let base_entity: Gd<Node2D> = Gd::from_instance_id(game_scene_id.0);
	let starting_position = base_entity.get_node_as::<Marker2D>("StartPosition");
	let mut animated_sprite = player_entity.get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");
	animated_sprite.set_flip_v(false);
	animated_sprite.set_flip_h(false);
	animated_sprite.stop();
	player_entity.set_global_position(starting_position.get_position());
	player_entity.show();
	score_timer.0.reset();
	score.0 = 0;
	next_game_state.set(GameState::Starting);
    }
}

pub fn run_start_timer(mut start_timer: ResMut<StartTimer>, delta: Res<Delta>, mut next_game_state: ResMut<NextState<GameState>>) {
    start_timer.0.tick(Duration::from_secs_f64(delta.0));
    if start_timer.0.just_finished() {
	start_timer.0.reset();
	next_game_state.set(GameState::InGame);
    }
}

pub fn run_score_timer(mut score_timer: ResMut<ScoreTimer>, delta: Res<Delta>, mut score: ResMut<Score>) {
    score_timer.0.tick(Duration::from_secs_f64(delta.0));
    if score_timer.0.just_finished() {
	score.0 += 1;
    }
}
