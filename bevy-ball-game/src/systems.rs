use bevy::{app::AppExit, prelude::*, window::PrimaryWindow};

use crate::events::*;
use crate::AppState;

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn transition_to_game_state(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::G) {
        if app_state.0 != AppState::Game {
            app_state_next_state.set(AppState::Game);
            println!("Entered AppState::Game");
        }
    }
}

pub fn transition_to_main_menu_state(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::M) {
        if app_state.0 != AppState::MainMenu {
            app_state_next_state.set(AppState::MainMenu);
            println!("Entered AppState::MainMenu");
        }
    }
}

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

pub fn handle_game_over(mut commands: Commands, mut game_over_event_reader: EventReader<GameOver>) {
    for event in game_over_event_reader.iter() {
        println!("Game over! Final score: {}", event.score.to_string());
        commands.insert_resource(NextState(Some(AppState::GameOver)))
    }
}
