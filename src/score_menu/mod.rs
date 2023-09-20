pub mod components;
pub mod systems;
use std::time::Duration;

use systems::*;

use bevy::prelude::*;

use crate::game::components::SimulationState;

use self::components::ScoreTimer;

pub struct ScoreMenuPlugin;

impl Plugin for ScoreMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter State Systems
            .insert_resource(ScoreTimer {
                timer: Timer::new(Duration::from_secs(15), TimerMode::Once),
                index: 0,
            },)
            .add_systems(OnEnter(SimulationState::Score),(
                    spawn_score_menu,
                    reset_score_timer, 
                    enter_score_menu_sound,
            ))
            // Systems
            .add_systems(Update, (
                    interact_with_leave_button, 
                    score_animation
            ))
            // OnExit State System
            .add_systems(OnExit(SimulationState::Score), despawn_score_menu);
    }
}