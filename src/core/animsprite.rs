use bevy::prelude::*;

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct AnimationResetFlag(pub bool);

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    query.for_each_mut(|(indices, mut timer, mut sprite)| {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index >= indices.last {
                indices.first
            } else {
                sprite.index + 1
            }
        }
    });
}
