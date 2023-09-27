use crate::{
    component::{Actor, Voice},
    model::AudioPlay,
    resource::AudioTracker,
};
use bevy::{
    ecs::system::Res,
    math::Vec3Swizzles,
    prelude::{Query, ResMut, Transform},
    time::Time,
};

pub fn voice(
    mut actors: Query<(&Actor, &mut Voice, &Transform)>,
    mut audio: ResMut<AudioTracker>,
    time: Res<Time>,
) {
    let now = time.elapsed();

    for (actor, mut voice, transform) in actors.iter_mut() {
        if let Some(sound) = voice.pop_queued(now) {
            audio.queue(AudioPlay {
                path: format!("{}/{}", actor.config.kind.get_assets_path(), sound.name()).into(),
                source: Some(transform.translation.xy()),
                ..AudioPlay::DEFAULT
            });
        }
    }
}
