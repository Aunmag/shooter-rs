use crate::AppState;
use bevy::{
    app::App,
    ecs::{schedule::IntoScheduleConfigs, system::ScheduleSystem},
    prelude::Update,
    state::{condition::in_state, state::OnEnter},
};

pub trait AppExt {
    fn add_state_system<M>(
        &mut self,
        state: AppState,
        system: impl IntoScheduleConfigs<ScheduleSystem, M>,
    ) -> &mut Self;

    fn add_state_system_enter<M>(
        &mut self,
        state: AppState,
        system: impl IntoScheduleConfigs<ScheduleSystem, M>,
    ) -> &mut Self;
}

impl AppExt for App {
    fn add_state_system<M>(
        &mut self,
        state: AppState,
        system: impl IntoScheduleConfigs<ScheduleSystem, M>,
    ) -> &mut Self {
        return self.add_systems(Update, system.run_if(in_state(state)));
    }

    fn add_state_system_enter<M>(
        &mut self,
        state: AppState,
        system: impl IntoScheduleConfigs<ScheduleSystem, M>,
    ) -> &mut Self {
        return self.add_systems(OnEnter(state), system);
    }
}
