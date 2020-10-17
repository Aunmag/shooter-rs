use std::collections::HashMap;

pub type UiTaskResource = HashMap<String, UiTask>;

pub enum UiTask {
    SetButtonAvailability(bool),
    SetText(&'static str),
}
