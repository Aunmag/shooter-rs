pub trait TakeContent<T> {
    fn take_content(&mut self) -> T;
}

impl<T> TakeContent<Vec<T>> for Vec<T> {
    fn take_content(&mut self) -> Self {
        if self.is_empty() {
            return Self::new();
        } else {
            return std::mem::replace(self, Self::with_capacity(self.capacity()));
        }
    }
}
