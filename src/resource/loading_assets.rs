use bevy::asset::HandleUntyped;

#[derive(Default)]
pub struct LoadingAssets {
    pub assets: Vec<HandleUntyped>,
}
