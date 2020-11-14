use amethyst::input::BindingTypes;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum AxisBinding {
    MoveForward,
    MoveAside,
}

impl Display for AxisBinding {
    fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
        return write!(formatter, "{:?}", self);
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionBinding {
    Attack,
}

impl Display for ActionBinding {
    fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
        return write!(formatter, "{:?}", self);
    }
}

#[derive(Debug)]
pub struct CustomBindingTypes;

impl BindingTypes for CustomBindingTypes {
    type Axis = AxisBinding;
    type Action = ActionBinding;
}
