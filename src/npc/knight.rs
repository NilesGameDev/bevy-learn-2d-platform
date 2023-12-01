use crate::{
    core::animsprite::{AnimationIndices, AnimationTimer, AnimationResetFlag},
    resource::AppState,
};
use bevy::{asset::LoadedFolder, prelude::*};

const KNIGHT_SPEED: f32 = 400.0; // TODO: Move to a component?

#[derive(Component)]
pub struct Knight;

#[derive(Component, PartialEq, Eq, Hash)]
pub enum KnightState {
    Idle,
    Run,
    Attack,
}

#[derive(Resource, Default)]
pub struct KnightSpriteFolder(Handle<LoadedFolder>);

pub fn preload_sprite_atlases(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(KnightSpriteFolder(
        asset_server.load_folder("textures/sprites/characters/knight"),
    ));
}

pub fn check_loading_finished(
    mut next_state: ResMut<NextState<AppState>>,
    knight_folder: ResMut<KnightSpriteFolder>,
    mut events: EventReader<AssetEvent<LoadedFolder>>,
) {
    for event in events.read() {
        if event.is_loaded_with_dependencies(&knight_folder.0) {
            next_state.set(AppState::Finished);
        }
    }
}

pub fn setup_sprites_from_atlas(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
    knight_folder_res: Res<KnightSpriteFolder>,
    loaded_folders: Res<Assets<LoadedFolder>>,
) {
    let mut atlas_builder = TextureAtlasBuilder::default();
    let loaded_folder = loaded_folders.get(&knight_folder_res.0).unwrap();

    for handle in loaded_folder.handles.iter() {
        let id = handle.id().typed_unchecked::<Image>();
        let Some(sub_atlas) = textures.get(id) else {
            warn!(
                "{:?} did not resolve to an `Image` asset!",
                handle.path().unwrap()
            );
            continue;
        };

        atlas_builder.add_texture(id, sub_atlas);
    }

    let assemble_atlas = atlas_builder.finish(&mut textures).unwrap();
    let assemble_atlas = TextureAtlas::from_grid(
        assemble_atlas.texture,
        Vec2::new(120.0, 80.0),
        24,
        1,
        None,
        None,
    );
    let atlas_handle = texture_atlases.add(assemble_atlas);
    let anim_indices = AnimationIndices { first: 0, last: 9 }; // TODO: find proper value

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: atlas_handle,
            sprite: TextureAtlasSprite::new(anim_indices.first),
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        },
        anim_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        AnimationResetFlag(true),
        Knight,
        KnightState::Idle,
    ));
}

pub fn move_knight(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut KnightState), With<Knight>>,
) {
    let (mut knight_transform, mut knight_state) = query.single_mut();

    let mut direction = 0.0;
    let mut is_moving = false;

    if keyboard_input.pressed(KeyCode::A) {
        direction -= 1.0;
        is_moving = true;
    }

    if keyboard_input.pressed(KeyCode::D) {
        direction += 1.0;
        is_moving = true;
    }

    if is_moving {
        *knight_state = KnightState::Run;
    } else {
        *knight_state = KnightState::Idle;
    }

    let new_position =
        knight_transform.translation.x + direction * KNIGHT_SPEED * time.delta_seconds();

    knight_transform.translation.x = new_position;
}

pub fn flip_knight(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut TextureAtlasSprite, With<Knight>>,
) {
    let mut knight_atlas = query.single_mut();

    if keyboard_input.pressed(KeyCode::A) {
        knight_atlas.flip_x = true;
    }
    if keyboard_input.pressed(KeyCode::D) {
        knight_atlas.flip_x = false;
    }
}

pub fn attack(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut KnightState, With<Knight>>,
) {
    if keyboard_input.pressed(KeyCode::Space) {
        let mut knight_state = query.single_mut();
        *knight_state = KnightState::Attack
    }
}

pub fn update_knight_state(mut query: Query<(&KnightState, &mut AnimationIndices), With<Knight>>) {
    let (knight_state, mut anim_indices) = query.single_mut();

    match knight_state {
        KnightState::Idle => {
            anim_indices.first = 0;
            anim_indices.last = 9;
        }
        KnightState::Run => {
            anim_indices.first = 10;
            anim_indices.last = 19;
        }
        KnightState::Attack => {
            anim_indices.first = 20;
            anim_indices.last = 23;
        }
    }
}
