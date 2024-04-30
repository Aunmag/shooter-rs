use bevy::{math::Vec2, utils::HashMap};

#[rustfmt::skip]
const OFFSETS: &[(i32, i32)] = &[
    (-1, -1),
    ( 0, -1),
    ( 1, -1),
    (-1,  0),
    ( 0,  0),
    ( 1,  0),
    (-1,  1),
    ( 0,  1),
    ( 1,  1),
];

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkId {
    pub x: i32,
    pub y: i32,
}

impl From<Vec2> for ChunkId {
    fn from(position: Vec2) -> Self {
        return Self {
            x: position.x.floor() as i32,
            y: position.y.floor() as i32,
        };
    }
}

pub struct ChunkMap<T> {
    pub map: HashMap<ChunkId, Vec<T>>,
}

impl<T> ChunkMap<T> {
    pub fn new(capacity: usize) -> Self {
        return Self {
            map: HashMap::with_capacity(capacity),
        };
    }

    pub fn insert(&mut self, position: Vec2, value: T) {
        self.map
            .entry(ChunkId::from(position))
            .or_insert_with(Vec::new)
            .push(value);
    }

    pub fn pop(&mut self) -> Option<(T, ChunkId)> {
        let mut value = None;
        let mut empty_chunk_id = None;

        for (id, values) in self.map.iter_mut() {
            value = values.pop().map(|v| (v, *id));

            if value.is_none() || values.is_empty() {
                empty_chunk_id = Some(*id);
            }

            if value.is_some() {
                break;
            }
        }

        if let Some(id) = empty_chunk_id {
            self.map.remove(&id);
        }

        return value;
    }

    pub fn iter_neighbors<F: FnMut(&T)>(&self, center: ChunkId, mut f: F) {
        for offset in OFFSETS {
            let Some(chunk) = self.map.get(&ChunkId {
                x: center.x + offset.0,
                y: center.y + offset.1,
            }) else {
                continue;
            };

            for item in chunk.iter() {
                f(item);
            }
        }
    }
}
