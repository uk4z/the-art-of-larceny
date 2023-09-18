pub mod components;
pub mod systems;
use std::time::Duration;

use systems::*;

use bevy::prelude::*;

use crate::{game::components::SimulationState, AppState};

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
            .add_systems(
                (
                    spawn_score_menu,
                    reset_score_timer, 
                ).in_schedule(OnEnter(SimulationState::Score)))
            // Systems
            .add_systems(
                (interact_with_leave_button, score_animation )
                    .in_set(OnUpdate(SimulationState::Score))
                    .in_set(OnUpdate(AppState::Game)),
            )
            // OnExit State System
            .add_system(despawn_score_menu.in_schedule(OnExit(SimulationState::Score)));
    }
}