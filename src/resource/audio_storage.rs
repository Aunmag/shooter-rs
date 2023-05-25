use bevy::{
    asset::AssetServer,
    prelude::{Assets, AudioSource, Handle, Resource},
};
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;
use regex::{Captures, Regex};
use std::collections::HashMap;

lazy_static::lazy_static! {
    static ref RE: Regex = Regex::new(r"(_)(\d+)(\.ogg)$").expect("Failed to parse audio regex");
}

#[derive(Resource)]
pub struct AudioStorage {
    groups: HashMap<String, AudioGroup>,
    generator: Pcg32,
}

impl Default for AudioStorage {
    fn default() -> Self {
        return Self {
            groups: HashMap::new(),
            generator: Pcg32::seed_from_u64(193_330),
        };
    }
}

impl AudioStorage {
    pub fn index(&mut self, assets: &Assets<AudioSource>, asset_server: &AssetServer) {
        log::debug!("Indexing");
        self.groups.clear();

        for handle_id in assets.ids() {
            let handle = assets.get_handle(handle_id);

            if let Some(path) = asset_server.get_handle_path(handle.clone()) {
                let asset_path = path.path().display().to_string().replace('\\', "/");
                let group_path = RE.replace(&asset_path, |c: &Captures| {
                    return format!("{}{{n}}{}", &c[1], &c[3]);
                });

                log::debug!("Found {} as {}", asset_path, group_path);

                self.groups
                    .entry(group_path.into_owned())
                    .or_insert_with(AudioGroup::default)
                    .audios
                    .push(handle);
            }
        }

        self.validate();

        log::debug!("Indexed groups: {}", self.groups.len());
    }

    pub fn validate(&self) {
        for (name, group) in self.groups.iter() {
            let is_multiple = name.contains("{n}");

            match group.audios.len() {
                0 => {
                    log::warn!("Group \"{}\" has no audios", name);
                }
                1 => {
                    if is_multiple {
                        log::warn!("Group \"{}\" has only one audio", name);
                    }
                }
                n => {
                    if !is_multiple {
                        log::warn!("Group \"{}\" has more than one audio: {}", name, n);
                    }
                }
            }
        }
    }

    pub fn choose(&mut self, path: &str) -> Option<Handle<AudioSource>> {
        return self
            .groups
            .get_mut(path)
            .and_then(|c| c.choose(&mut self.generator));
    }
}

#[derive(Default)]
struct AudioGroup {
    audios: Vec<Handle<AudioSource>>,
    last_chosen_index: usize,
}

impl AudioGroup {
    fn choose(&mut self, generator: &mut Pcg32) -> Option<Handle<AudioSource>> {
        match self.audios.len() {
            0 => {
                return None;
            }
            1 => {
                return self.get(0);
            }
            2 => {
                return self.get(self.last_chosen_index + 1);
            }
            len => {
                let mut index = generator.gen_range(0..len);

                if index == self.last_chosen_index {
                    index += 1;
                }

                return self.get(index);
            }
        }
    }

    fn get(&mut self, mut index: usize) -> Option<Handle<AudioSource>> {
        index %= self.audios.len();
        self.last_chosen_index = index;
        return self.audios.get(index).cloned();
    }
}
