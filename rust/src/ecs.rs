use bevy_ecs::prelude::*;
pub struct ECS {
    pub world: World,
    pub schedule: Schedule,
}

impl ECS {
    pub fn new() -> Self {
	ECS {
	    world: World::new(),
	    schedule: Schedule::default(),
	}
    }

    pub fn add_systems<M>(&mut self, systems: impl IntoSystemConfigs<M>) -> &mut Self {
	self.schedule.add_systems(systems);
	self
    }

    pub fn run(&mut self) {
	self.schedule.run(&mut self.world);
    }
}
