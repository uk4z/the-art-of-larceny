use bevy::prelude::*;

use super::components::*;

pub fn spawn_security(
    mut commands: Commands,
) {
    commands.spawn((
        SecurityBundle {
            intrusion: Intrusion(false),
        },
        Security,
    )); 
}