use enumset::EnumSet;
use enumset::EnumSetType;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize, EnumSetType)]
pub enum ActorAction {
    MovementForward,
    MovementBackward,
    MovementLeftward,
    MovementRightward,
    Sprint,
    Attack,
}

pub type ActorActions = EnumSet<ActorAction>;

pub trait ActorActionsExt {
    const MOVEMENT: ActorActions = enumset::enum_set!(
        ActorAction::MovementForward
            | ActorAction::MovementBackward
            | ActorAction::MovementLeftward
            | ActorAction::MovementRightward
    );

    fn clean(self) -> Self;

    fn set(&mut self, value: ActorAction, state: bool);

    #[allow(clippy::wrong_self_convention)]
    fn is_moving(self) -> bool;

    #[allow(clippy::wrong_self_convention)]
    fn is_attacking(self) -> bool;

    #[allow(clippy::wrong_self_convention)]
    fn is_sprinting(self) -> bool;
}

impl ActorActionsExt for ActorActions {
    fn clean(self) -> Self {
        if self.is_moving() {
            return self;
        } else {
            return self - ActorAction::Sprint;
        }
    }

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

    fn is_attacking(self) -> bool {
        return self.contains(ActorAction::Attack);
    }

    fn is_sprinting(self) -> bool {
        return self.contains(ActorAction::Sprint);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size() {
        assert_eq!(1, std::mem::size_of::<ActorActions>());
        assert_eq!(6, ActorActions::bit_width());
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
        assert_eq!(actions.len(), 6);
        assert_eq!(actions.as_u8(), 0b111111);
        assert_eq!(actions.as_u8(), 63);
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
