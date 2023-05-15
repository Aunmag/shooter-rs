use enumset::EnumSet;
use enumset::EnumSetType;

pub trait EnumSetExt<T: EnumSetType> {
    fn set(&mut self, value: T, state: bool);
}

impl<T: EnumSetType> EnumSetExt<T> for EnumSet<T> {
    fn set(&mut self, value: T, state: bool) {
        if state {
            self.insert(value);
        } else {
            self.remove(value);
        }
    }
}
