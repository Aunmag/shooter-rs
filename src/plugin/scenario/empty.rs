use crate::plugin::scenario::ScenarioLogic;
use std::any::Any;

pub struct EmptyScenario;

impl ScenarioLogic for EmptyScenario {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        return self;
    }
}
