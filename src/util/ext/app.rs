use crate::model::AppState;
use bevy::{
    app::App,
    ecs::schedule::OnEnter,
    prelude::{in_state, IntoSystemConfigs, Update},
};

pub trait AppExt {
    fn add_state_system<M>(
        &mut self,
        state: AppState,
        system: impl IntoSystemConfigs<M>,
    ) -> &mut Self;

    fn add_state_systems(&mut self, state: AppState, f: fn(&mut StateSystems)) -> &mut Self;

    fn add_state_system_enter<M>(
        &mut self,
        state: AppState,
        system: impl IntoSystemConfigs<M>,
    ) -> &mut Self;
}

impl AppExt for App {
    fn add_state_system<M>(
        &mut self,
        state: AppState,
        system: impl IntoSystemConfigs<M>,
    ) -> &mut Self {
        return self.add_systems(Update, system.run_if(in_state(state)));
    }

    fn add_state_systems(&mut self, state: AppState, f: fn(&mut StateSystems)) -> &mut Self {
        f(&mut StateSystems { app: self, state });
        return self;
    }

    fn add_state_system_enter<M>(
        &mut self,
        state: AppState,
        system: impl IntoSystemConfigs<M>,
    ) -> &mut Self {
        return self.add_systems(OnEnter(state), system);
    }
}

pub struct StateSystems<'a> {
    app: &'a mut App,
    state: AppState,
}

impl<'a> StateSystems<'a> {
    pub fn add<M, S: IntoSystemConfigs<M>>(&mut self, system: S) {
        self.app.add_state_system(self.state, system);
    }
}
