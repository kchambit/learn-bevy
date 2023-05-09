use bevy::prelude::*;

pub mod resources;
mod systems;

use resources::*;
use systems::*;

use crate::AppState;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            //Resources
            .init_resource::<HighScores>()
            //On enter state
            .add_system(insert_score.in_schedule(OnEnter(AppState::Game)))
            //Systems
            .add_system(update_score.run_if(in_state(AppState::Game)))
            .add_system(update_high_scores)
            .add_system(high_scores_updated)
            //On exit state
            .add_system(remove_score.in_schedule(OnExit(AppState::Game)));
    }
}
