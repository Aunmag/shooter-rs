use crate::{
    command::ActorRelease,
    component::{Voice, VoiceSound},
    event::ActorDeathEvent,
};
use bevy::{
    ecs::system::{Query, Res},
    prelude::{Commands, EventReader},
    time::Time,
};

pub fn on_actor_death(
    mut events: EventReader<ActorDeathEvent>,
    mut voices: Query<&mut Voice>,
    mut commands: Commands,
    time: Res<Time>,
) {
    let now = time.elapsed();

    for event in events.iter() {
        // TODO: find a way to stop all other voices

        if let Ok(mut voice) = voices.get_mut(event.entity) {
            voice.queue(VoiceSound::Death, now);
        }

        if let Some(attacker) = event.attacker {
            if let Ok(mut voice) = voices.get_mut(attacker) {
                voice.queue(VoiceSound::Kill, now);
            }
        }

        commands.add(ActorRelease(event.entity));
        // TODO: maybe call despawn from here?
    }
}
