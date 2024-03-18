use bevy_ecs::prelude::*;
use bevy_time::Timer;
use godot::prelude::*;

#[derive(Resource, Default)]
pub struct Delta(pub f64);

#[derive(Resource)]
pub struct ScreenSize(pub Vector2);

#[derive(Resource)]
pub struct GameSceneInstanceId(pub InstanceId);

#[derive(Resource)]
pub struct MobScene(pub InstanceId);

#[derive(Resource)]
pub struct MobTimer(pub Timer);

#[derive(Resource)]
pub struct ScoreTimer(pub Timer);

#[derive(Resource)]
pub struct StartTimer(pub Timer);

#[derive(Resource)]
pub struct Score(pub u32);

#[derive(Event)]
pub struct StartGameEvent;

#[derive(Event)]
pub struct InMenuEvent;

#[derive(GodotConvert, Var, Export, Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
#[godot(via = GString)]
pub enum GameState {
    #[default]
    Menu,
    Starting,
    InGame,
    GameOver,
}
