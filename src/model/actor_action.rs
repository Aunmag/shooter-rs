use enumset::{EnumSet, EnumSetType};

#[derive(Debug, EnumSetType)]
pub enum ActorAction {
    Sprint,
    Attack,
    Reload,
}

pub type ActorActions = EnumSet<ActorAction>;

#[allow(clippy::wrong_self_convention)]
pub trait ActorActionsExt {
    fn set(&mut self, value: ActorAction, state: bool);
    fn is_sprinting(self) -> bool;
    fn is_attacking(self) -> bool;
    fn is_reloading(self) -> bool;
}

impl ActorActionsExt for ActorActions {
    fn set(&mut self, value: ActorAction, state: bool) {
        if state {
            self.insert(value);
        } else {
            self.remove(value);
        }
    }

    fn is_sprinting(self) -> bool {
        return self.contains(ActorAction::Sprint);
    }

    fn is_attacking(self) -> bool {
        return self.contains(ActorAction::Attack);
    }

    fn is_reloading(self) -> bool {
        return self.contains(ActorAction::Reload);
    }
}
