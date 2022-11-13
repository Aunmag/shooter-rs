use crate::resource::Config;
use crate::resource::GameType;
use bevy::prelude::World;

pub trait WorldExt {
    fn config(&self) -> &Config;
    fn is_server(&self) -> bool;
    fn is_client(&self) -> bool;
}

impl WorldExt for World {
    fn config(&self) -> &Config {
        return self.resource::<Config>();
    }

    fn is_server(&self) -> bool {
        return self.resource::<GameType>().is_server();
    }

    fn is_client(&self) -> bool {
        return self.resource::<GameType>().is_client();
    }
}
