use crate::model::AppState;
use bevy::app::App;
use bevy::ecs::schedule::OnEnter;
use bevy::ecs::schedule::OnUpdate;
use bevy::prelude::IntoSystemAppConfig;
use bevy::prelude::IntoSystemConfig;

pub trait AppExt {
    fn add_state_system<M>(
        &mut self,
        state: AppState,
        system: impl IntoSystemAppConfig<M> + IntoSystemConfig<M>,
    ) -> &mut Self;

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

    fn add_state_system_enter<M>(
        &mut self,
        state: AppState,
        system: impl IntoSystemAppConfig<M> + IntoSystemConfig<M>,
    ) -> &mut Self {
        self.add_system(system.in_schedule(OnEnter(state)))
    }
}
