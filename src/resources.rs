pub type UiTaskResource = Vec<UiTask>;

pub enum UiTask {
    SetButtonAvailability(&'static str, bool),
}
