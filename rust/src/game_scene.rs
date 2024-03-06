use godot::{prelude::*, engine::Area2D};
use crate::ecs::ECS;
use crate::components::{GodotInstanceId};
use crate::player::{PlayerBundle, process_player_input, PlayerSpeed, move_entities, animate_player};
use crate::resources::{Delta, ScreenSize};

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct GameScene {
    pub ecs: ECS,
    pub base: Base<Node2D>,
}

#[godot_api]
impl INode for GameScene {
    fn init(base: Base<Node2D>) -> Self {
	let ecs = ECS::new();
	let game_scene = Self { ecs, base };
	game_scene
    }

    fn ready(&mut self) {
	let player_scene: Gd<PackedScene> = load("res://player.tscn");
	let mut player_instance = player_scene.instantiate_as::<Area2D>();
	let player_id = player_instance.instance_id();
	//player_instance.hide();
	self.base_mut().add_child(player_instance.upcast());
	self.ecs.world.spawn(PlayerBundle {
	    instance_id: GodotInstanceId(player_id),
	    ..Default::default()
	});
	self.ecs.world.insert_resource(PlayerSpeed { value: 400.0 });
	self.ecs.world.insert_resource(Delta::default());
	let screen_size = self.base().get_viewport_rect().size;
	self.ecs.world.insert_resource(ScreenSize(screen_size));
	self.ecs.add_systems((process_player_input, move_entities, animate_player));
	godot_print!("here!");
    }
    fn process(&mut self, delta: f64) {
	let mut delta_res = self.ecs.world.get_resource_mut::<Delta>().unwrap();
	delta_res.0 = delta;
	self.ecs.run();
    }
}

#[godot_api]
impl GameScene {
    
}
