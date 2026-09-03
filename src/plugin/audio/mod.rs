mod audio_play;
mod audio_pool;
mod audio_storage;

pub use self::{audio_play::*, audio_pool::*, audio_storage::*};
use bevy::{
    app::Update,
    prelude::{App, Plugin},
};

pub struct AudioPlugin {
    limit: usize,
}

impl AudioPlugin {
    pub fn new(limit: usize) -> Self {
        return Self { limit };
    }
}

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AudioStorage::default());
        app.insert_resource(AudioPool::new(self.limit));
        app.add_systems(Update, audio_pool::on_update);
    }
}
