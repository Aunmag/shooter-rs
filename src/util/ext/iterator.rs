pub trait IteratorExt<T: Copy> {
    fn closest<F>(self, distance_fn: F) -> Option<T>
    where
        F: Fn(T) -> f32;
}

impl<T: Copy, I: IntoIterator<Item = T>> IteratorExt<T> for I {
    fn closest<F>(self, distance_fn: F) -> Option<T>
    where
        F: Fn(T) -> f32,
    {
        let mut closest: Option<(T, f32)> = None;

        for item in self.into_iter() {
            let distance = distance_fn(item);

            if closest.map_or(true, |c| c.1 > distance) {
                closest = Some((item, distance));
            }
        }

        return closest.map(|c| c.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_closest() {
        let f = |a: u8, b: f32| (a as f32 - b).abs();
        let numbers = [0, 9, 1, 8, 2, 7, 3, 6, 4, 5];
        assert_eq!(numbers.iter().closest(|n| f(*n, 10.0)), Some(&9));
        assert_eq!(numbers.iter().closest(|n| f(*n, -10.0)), Some(&0));
        assert_eq!(numbers.iter().closest(|n| f(*n, 3.2)), Some(&3));
        assert_eq!(numbers.iter().closest(|n| f(*n, 3.8)), Some(&4));
        assert_eq!([].iter().closest(|n| f(*n, 3.8)), None);
    }
}
