use enumset::{EnumSet, EnumSetType};

#[derive(Debug, EnumSetType)]
pub enum ActorAction {
    MovementForward,
    MovementBackward,
    MovementLeftward,
    MovementRightward,
    Sprint,
    Attack,
    Reload,
}

pub type ActorActions = EnumSet<ActorAction>;

pub trait ActorActionsExt {
    const MOVEMENT: ActorActions = enumset::enum_set!(
        ActorAction::MovementForward
            | ActorAction::MovementBackward
            | ActorAction::MovementLeftward
            | ActorAction::MovementRightward
    );

    fn set(&mut self, value: ActorAction, state: bool);

    #[allow(clippy::wrong_self_convention)]
    fn is_moving(self) -> bool;

    #[allow(clippy::wrong_self_convention)]
    fn is_sprinting(self) -> bool;

    #[allow(clippy::wrong_self_convention)]
    fn is_attacking(self) -> bool;

    #[allow(clippy::wrong_self_convention)]
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

    fn is_moving(self) -> bool {
        return !self.is_disjoint(Self::MOVEMENT);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size() {
        assert_eq!(1, std::mem::size_of::<ActorActions>());
        assert_eq!(7, ActorActions::bit_width());
    }

    #[test]
    fn test_bits_empty() {
        let actions = ActorActions::EMPTY;
        assert_eq!(actions.len(), 0);
        assert_eq!(actions.as_u8(), 0b0);
        assert_eq!(actions.as_u8(), 0);
    }

    #[test]
    fn test_bits_full() {
        let actions = ActorActions::ALL;
        assert_eq!(actions.len(), 7);
        assert_eq!(actions.as_u8(), 0b1111111);
        assert_eq!(actions.as_u8(), 127);
    }

    #[test]
    fn test_bits_complex() {
        let actions = ActorAction::MovementLeftward | ActorAction::Sprint;
        assert_eq!(actions.len(), 2);
        assert_eq!(actions.as_u8(), 0b10100);
        assert_eq!(actions.as_u8(), 20);
    }

    #[test]
    fn test_is_moving() {
        assert!(!ActorActions::EMPTY.is_moving());
        assert!(ActorActions::ALL.is_moving());
        assert!(ActorActions::only(ActorAction::MovementForward).is_moving());
        assert!(ActorActions::only(ActorAction::MovementBackward).is_moving());
        assert!(ActorActions::only(ActorAction::MovementLeftward).is_moving());
        assert!(ActorActions::only(ActorAction::MovementRightward).is_moving());
        assert!(!ActorActions::only(ActorAction::Sprint).is_moving());
        assert!(!ActorActions::only(ActorAction::Attack).is_moving());
    }
}
