use bevy::{
    asset::AssetServer,
    prelude::{Assets, AudioSource, Handle, Resource},
};
use rand::{seq::SliceRandom, SeedableRng};
use rand_pcg::Pcg32;
use regex::Regex;
use std::collections::{HashMap, HashSet};

lazy_static::lazy_static! {
    static ref RE: Regex = Regex::new(r"(_+\d+)?\.ogg$").expect("Failed to parse audio regex");
}

const SPARE_PATHS: &[(&str, &str)] = &[
    ("actors/zombie_agile/", "actors/zombie/"),
];

#[derive(Resource)]
pub struct AudioStorage {
    groups: HashMap<String, AudioGroup>,
    missing: HashSet<String>,
    generator: Pcg32,
}

impl Default for AudioStorage {
    fn default() -> Self {
        return Self {
            groups: HashMap::new(),
            missing: HashSet::new(),
            generator: Pcg32::seed_from_u64(193_330),
        };
    }
}

impl AudioStorage {
    pub fn index(&mut self, assets: &Assets<AudioSource>, asset_server: &AssetServer) {
        log::debug!("Indexing");
        self.groups.clear();
        self.missing.clear();

        for asset_id in assets.ids() {
            if let Some(handle) = asset_server.get_id_handle(asset_id) {
                if let Some(path) = handle.path() {
                    let asset_path = path.path().display().to_string().replace('\\', "/");
                    let group_path = RE.replace_all(&asset_path, "");

                    self.groups
                        .entry(group_path.into_owned())
                        .or_insert_with(AudioGroup::default)
                        .audios
                        .push(handle);
                }
            }
        }

        log::debug!("Indexed groups: {}", self.groups.len());
    }

    pub fn choose(&mut self, path: &str) -> Option<Handle<AudioSource>> {
        let mut audio = self.choose_exact(path);

        if audio.is_none() && !self.missing.contains(path) {
            for spare in SPARE_PATHS {
                if path.starts_with(spare.0) {
                    audio = self.choose_exact(&path.replace(spare.0, spare.1));
                    break;
                }
            }

            if audio.is_none() {
                self.missing.insert(path.to_string());
                log::warn!("Audio {} not found", path);
            }
        }

        return audio;
    }

    fn choose_exact(&mut self, path: &str) -> Option<Handle<AudioSource>> {
        return self
            .groups
            .get_mut(path)
            .and_then(|g| g.choose(&mut self.generator))
            .cloned();
    }
}

#[derive(Default)]
struct AudioGroup {
    audios: Vec<Handle<AudioSource>>,
    cursor: usize,
}

impl AudioGroup {
    fn choose(&mut self, generator: &mut Pcg32) -> Option<&Handle<AudioSource>> {
        self.cursor = (self.cursor + 1) % self.audios.len();

        if self.cursor == 0 && self.audios.len() > 2 {
            self.audios.shuffle(generator);
        }

        return self.audios.get(self.cursor);
    }
}
