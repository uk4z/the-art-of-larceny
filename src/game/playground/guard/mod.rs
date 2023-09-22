pub mod systems;
pub mod components;

use bevy::prelude::*;

use crate::AppState;
use crate::game::SimulationState;
use crate::game::playground::components::WorldPosition;
use crate::game::playground::guard::components::Patrol; 

use systems::*;

use super::scenery::components::{Bounds, ScenerySize};




pub struct GuardPlugin;

impl Plugin for GuardPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(SimulationState::Loading), spawn_guard)
            .add_systems(Update, 
                (
                    move_guard, 
                    update_patrols_positions,
                    guard_motion_handler,
                    guard_sound_handler,
                    handle_sound_distance.after(guard_sound_handler),  
                    update_waiting_timer,
                    alert_guard,
                    disalert_guard,
                    update_chase_stack,
                    catch_player,
                    bounds_guard, 
            ).run_if(in_state(SimulationState::Running))
            )
            .add_systems(PostUpdate, avoid_guard_collisions.run_if(in_state(SimulationState::Running)))
            .add_systems(OnEnter(SimulationState::Score), stop_footsteps)
            .add_systems(OnEnter(SimulationState::Paused), pause_footsteps)
            .add_systems(OnExit(AppState::Game), despawn_guard);
    }
}

pub fn patrol_direction( 
    patrol: &mut Patrol,
    guard_position: &WorldPosition, 
) -> Vec3 {
    let direction = Vec3::from(patrol.get_position())-Vec3::from(*guard_position);
    direction.normalize_or_zero()
}

pub fn chase_direction ( 
    player_pos: &WorldPosition,
    guard_pos: &WorldPosition, 
) -> Vec3 {
    let direction = Vec3::from(*player_pos) - Vec3::from(*guard_pos);
    direction.normalize_or_zero()
}

pub fn search_direction ( 
    target_pos: &WorldPosition,
    guard_pos: &WorldPosition, 
) -> Vec3 {
    let direction = Vec3::from(*target_pos) - Vec3::from(*guard_pos);
    direction.normalize_or_zero()
}

pub fn obstacle_in_fov (
    player_pos: &WorldPosition, 
    guard_pos: &WorldPosition, 
    bounds: &Bounds, 
    size: &ScenerySize, 
) -> bool {
    if !bounds.0.is_empty() {
        let direction = Vec3::from(*player_pos) - Vec3::from(*guard_pos); 
        for i in 0..1000 {
            let position = Vec3::from(*guard_pos) + (i as f32)*direction*0.001;
            let (x, y) = (position.x as usize, (size.height-position.y) as usize);
            if x < size.width as usize && y < size.height as usize && bounds.0[y][x] == 1 {
                return true;
            }
        }
        return false; 
    }
    true
}