use bevy::prelude::*;
use platformer_2d::core::{animsprite, maincamera};
use platformer_2d::npc::knight;
use platformer_2d::resource::AppState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_state::<AppState>()
        .add_systems(OnEnter(AppState::Setup), knight::preload_sprite_atlases)
        .add_systems(
            Update,
            knight::check_loading_finished.run_if(in_state(AppState::Setup)),
        )
        .add_systems(
            OnEnter(AppState::Finished),
            knight::setup_sprites_from_atlas,
        )
        .add_systems(Startup, maincamera::setup_camera)
        .add_systems(
            Update,
            (
                knight::move_knight.run_if(in_state(AppState::Finished)),
                knight::flip_knight.run_if(in_state(AppState::Finished)),
                knight::attack.run_if(in_state(AppState::Finished)),
                knight::update_knight_state.run_if(in_state(AppState::Finished)),
                animsprite::animate_sprite.run_if(in_state(AppState::Finished)),
            ),
        )
        .run();
}
