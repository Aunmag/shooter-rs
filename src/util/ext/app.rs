use crate::model::AppState;
use bevy::{
    app::App,
    ecs::schedule::{OnEnter, OnUpdate},
    prelude::{IntoSystemAppConfig, IntoSystemConfig},
};

pub trait AppExt {
    fn add_state_system<M>(
        &mut self,
        state: AppState,
        system: impl IntoSystemAppConfig<M> + IntoSystemConfig<M>,
    ) -> &mut Self;

    fn add_state_systems(&mut self, state: AppState, f: fn(&mut StateSystems)) -> &mut Self;

    fn add_state_system_enter<M>(
        &mut self,
        state: AppState,
        system: impl IntoSystemAppConfig<M> + IntoSystemConfig<M>,
    ) -> &mut Self;
}

impl AppExt for App {
    fn add_state_system<M>(
        &mut self,
        state: AppState,
        system: impl IntoSystemAppConfig<M> + IntoSystemConfig<M>,
    ) -> &mut Self {
        self.add_system(system.in_set(OnUpdate(state)))
    }

    fn add_state_systems(&mut self, state: AppState, f: fn(&mut StateSystems)) -> &mut Self {
        f(&mut StateSystems { app: self, state });
        return self;
    }

    fn add_state_system_enter<M>(
        &mut self,
        state: AppState,
        system: impl IntoSystemAppConfig<M> + IntoSystemConfig<M>,
    ) -> &mut Self {
        self.add_system(system.in_schedule(OnEnter(state)))
    }
}

pub struct StateSystems<'a> {
    app: &'a mut App,
    state: AppState,
}

impl<'a> StateSystems<'a> {
    pub fn add<M>(&mut self, system: impl IntoSystemAppConfig<M> + IntoSystemConfig<M>) {
        self.app.add_state_system(self.state, system);
    }
}
