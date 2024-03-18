use bevy_ecs::event::{Event, Events};
use bevy_ecs::schedule::States;
use godot::engine::CanvasLayer;
use godot::{prelude::*, engine::Area2D};
use crate::components::*;
use crate::player::*;
use crate::mob::*;
use crate::resources::*;
use bevy_time::{Timer, TimerMode};
use bevy_app::prelude::*;
use std::time::Duration;
use bevy_ecs::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct GameScene {
    pub ecs: App,
    pub base: Base<Node2D>,
    pub mob_scene: Gd<PackedScene>,
    #[var]
    pub score: u32,
}

#[godot_api]
impl INode for GameScene {
    fn init(base: Base<Node2D>) -> Self {
	let ecs = App::new();
	let game_scene = Self { ecs, base, mob_scene: PackedScene::new_gd(), score: 0};
	game_scene
    }

    fn ready(&mut self) {
	self.spawn_player();
	self.create_mob_scene();
	self.add_resources();
	self.add_systems();
	self.connect_signals();
	godot_print!("here!");
    }
    fn process(&mut self, delta: f64) {
	let mut delta_res = self.ecs.world.get_resource_mut::<Delta>().unwrap();
	delta_res.0 = delta;
	self.ecs.update();
	self.score = self.ecs.world.get_resource::<Score>().unwrap().0;
    }
}

#[godot_api]
impl GameScene {
    fn spawn_player(&mut self) {
	let player_scene: Gd<PackedScene> = load("res://player.tscn");
	let mut player_instance = player_scene.instantiate_as::<Area2D>();
	let player_id = player_instance.instance_id();
	player_instance.hide();
	self.base_mut().add_child(player_instance.upcast());
	self.ecs.world.spawn(PlayerBundle {
	    instance_id: GodotInstanceId(player_id),
	    ..Default::default()
	});
    }

    fn create_mob_scene(&mut self) {
	let mob_scene: Gd<PackedScene> = load("res://mob.tscn");
	self.mob_scene = mob_scene;
	self.ecs.world.insert_resource(MobScene(self.mob_scene.instance_id()));
    }

    fn add_resources(&mut self) {
	self.ecs.world.insert_resource(PlayerSpeed { value: 400.0 });
	self.ecs.world.insert_resource(Delta::default());
	let screen_size = self.base().get_viewport_rect().size;
	self.ecs.world.insert_resource(ScreenSize(screen_size));
	self.ecs.world.insert_resource(GameSceneInstanceId(self.base().instance_id()));	
	self.ecs.world.insert_resource(MobTimer(Timer::new(Duration::from_millis(500), TimerMode::Repeating)));
	self.ecs.world.insert_resource(ScoreTimer(Timer::new(Duration::from_secs(1), TimerMode::Repeating)));
	self.ecs.world.insert_resource(StartTimer(Timer::new(Duration::from_secs(2), TimerMode::Once)));
	self.ecs.world.insert_resource(Score(0));
	self.ecs.insert_state(GameState::Menu);
    }

    fn connect_signals(&mut self) {
	let mut hud = self.base_mut().get_node_as::<CanvasLayer>("HUD");
	hud.connect("start_game".into(), self.base().callable("on_start_game"));
	self.ecs.add_event::<StartGameEvent>();

	hud.connect("in_main_menu".into(), self.base().callable("on_main_menu"));
	self.ecs.add_event::<InMenuEvent>();
    }

    fn add_systems(&mut self) {
	self.ecs.add_systems(Update, (
			     (start_game).run_if(in_state(GameState::Menu)),
			     (run_start_timer).run_if(in_state(GameState::Starting)),
			     (process_player_input, move_entities, animate_player, spawn_mobs, detect_player_collisions, run_score_timer).run_if(in_state(GameState::InGame)),
			     (despawn_mobs, wait_for_start_signal).run_if(in_state(GameState::GameOver)),
	));			     
    }

    #[func]
    fn on_start_game(&mut self) {
	godot_print!("hhhh");
	let mut events = self.ecs.world.get_resource_mut::<Events<StartGameEvent>>().unwrap();
	events.send(StartGameEvent);
    }

    #[func]
    fn on_main_menu(&mut self) {
	let mut events = self.ecs.world.get_resource_mut::<Events<InMenuEvent>>().unwrap();
	events.send(InMenuEvent);
    }
}
