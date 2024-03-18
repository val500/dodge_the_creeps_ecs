use bevy_ecs::prelude::*;
use bevy_app::prelude::*;
use crate::resources::{GameState, NextGameState};

#[derive(Default)]
pub struct ECS {
    pub app: App,
}

impl ECS {
    pub fn new() -> Self {
	ECS::default()
    }

    pub fn add_systems<M>(&mut self, systems: impl IntoSystemConfigs<M>, game_state: &GameState) -> &mut Self {
	match game_state {
	    GameState::Menu => self.menu_schedule.add_systems(systems),
	    GameState::InGame => self.game_schedule.add_systems(systems),
	    GameState::GameOver => self.game_over_schedule.add_systems(systems),
	};
	self
    }

    pub fn run(&mut self) {
	match self.world.get_resource::<GameState>().unwrap() {
	    GameState::Menu => {
		self.menu_schedule.run(&mut self.world);
	    }
	    GameState::InGame => {
		self.game_schedule.run(&mut self.world);
	    }
	    GameState::GameOver => {
		self.game_over_schedule.run(&mut self.world);
	    }
	}
	*self.world.get_resource_mut::<GameState>().unwrap() = (*self.world.get_resource::<NextGameState>().unwrap()).into();
    }
}
