use bevy::prelude::*;
use crate::game::playground::components::{WorldPosition, Orientation, AnimatedMotion, ReachDistance};

pub const POSITION_REACH: f32 = 10.0;

#[derive(Component, Debug)]
pub struct Guard; 

#[derive(Component, Debug)]
pub struct Waiting {
    pub position: WorldPosition,
    pub time: Timer, 
}


#[derive(Component, Debug)]
pub struct Patrol {
    pub positions: Vec<WorldPosition>,
    pub position_index: usize, 
    pub waitings: Vec<Waiting>,
    pub waiting_index: usize,  
}

impl Patrol {
    pub fn get_position(&self) -> WorldPosition{ 
        self.positions[self.position_index]
    }

    pub fn get_waiting(&mut self) -> &mut Waiting {
        &mut self.waitings[self.waiting_index]
    }

    pub fn next_position(&mut self) {
        let length = self.positions.len();
        self.position_index = (self.position_index+1)%length;
    }

    pub fn next_waiting(&mut self) {
        let length = self.waitings.len();
        self.waiting_index = (self.waiting_index+1)%length;
    }

    pub fn is_waiting_position(&mut self) -> bool{
        (Vec3::from(self.get_position())-Vec3::from(self.get_waiting().position)).length() < 1.0
    }

    pub fn patrol_position_reached(&self, guard_pos: WorldPosition) -> bool{
        let distance = (Vec3::from(self.get_position())-Vec3::from(guard_pos)).length();
        distance < POSITION_REACH
    } 
}

#[derive(Component, Debug, Clone)]
pub enum GuardState {
    Patrolling,
    Chasing,
    Returning, 
    Searching(WorldPosition), 
    Waiting(Timer),
}

#[derive(Component, Debug)]
pub struct ChaseStack(pub Vec<(WorldPosition, Orientation)>);

#[derive(Debug, Component)]
pub enum GuardPace {
    Walk, 
    Run, 
}

impl Into<f32> for GuardPace {
    fn into(self) -> f32 {
        match self {
            GuardPace::Run => {
                1.2
            },
            GuardPace::Walk => {
                0.6
            },
        }
    }
}

#[derive(Bundle, Debug)]
pub struct GuardBundle {
    pub position: WorldPosition,
    pub orientation: Orientation, 
    pub pace: GuardPace,
    pub animation: AnimatedMotion,
    pub reach: ReachDistance,
    pub patrol: Patrol,
    pub state: GuardState,
    pub chase_stack: ChaseStack,
}