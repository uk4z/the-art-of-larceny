pub mod systems;
pub mod components;

use bevy::prelude::*;

use crate::AppState;
use crate::game::SimulationState;
use crate::game::playground::components::WorldPosition;
use crate::game::playground::guard::components::Patrol; 

use systems::*;

use super::scenery::SCENERY_SIZE;
use super::scenery::components::Bounds;




pub struct GuardPlugin;

impl Plugin for GuardPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(spawn_guard.in_schedule(OnEnter(AppState::Game)))
            .add_systems(
                (
                    move_guard, 
                    update_patrols_positions,
                    guard_motion_handler,
                    update_waiting_timer,
                    update_fov,
                    alert_guard,
                    disalert_guard,
                    update_chase_stack,
                    catch_player,
                ) 
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            );
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
) -> bool {
    if !bounds.0.is_empty() {
        let direction = Vec3::from(*player_pos) - Vec3::from(*guard_pos); 
        for i in 0..1000 {
            let position = Vec3::from(*guard_pos) + (i as f32)*direction*0.001;
            let (x, y) = (position.x as usize, (SCENERY_SIZE.1-position.y) as usize);
            if bounds.0[y][x] == 1 {
                return true;
            }
        }
        return false; 
    }
    true
}