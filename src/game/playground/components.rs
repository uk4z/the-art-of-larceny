use bevy::prelude::*;

#[derive(Component, Copy, Clone, Debug, PartialEq)]
pub struct WorldPosition {
    pub x: f32, 
    pub y: f32, 
}

impl From<WorldPosition> for Vec3 {
    fn from(value: WorldPosition) -> Self {
        Vec3::new(value.x, value.y, 0.0)
    }
}

#[derive(Component, Debug)]
pub struct ReachDistance(pub f32);

#[derive(Component, Debug, Clone, Copy)]
pub struct Orientation(pub Quat);

#[derive(Component, Debug)]
pub struct AnimatedMotion{
    pub walk_timer: Timer,
    pub run_timer: Timer,
}

#[derive(Debug, Component)]
pub struct Name(pub String);

#[derive(Debug, Component)]
pub struct Value(pub f32);