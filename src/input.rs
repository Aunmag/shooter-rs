use amethyst::input::BindingTypes;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::sync::atomic::AtomicI16;
use std::sync::atomic::Ordering;

// TODO: Try make an object with methods for it
// TODO: Make sure it's a good way to track mouse movement with minimal delay
// TODO: Maybe it's better to store a decimal value, at least I've seen integers only so far
static MOUSE_DELTA: AtomicI16 = AtomicI16::new(0);

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

pub fn reset_mouse_delta() {
    MOUSE_DELTA.store(0, Ordering::Relaxed);
}

pub fn add_mouse_delta(value: i16) {
    // TODO: Make sure it's a proper way to do an atomic saturating addition
    MOUSE_DELTA.store(
        MOUSE_DELTA.load(Ordering::Relaxed).saturating_add(value),
        Ordering::Relaxed,
    );
}

pub fn take_mouse_delta() -> i16 {
    return MOUSE_DELTA.swap(0, Ordering::Relaxed);
}
