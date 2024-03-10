use crate::util::SmartString;
use bevy::{
    asset::{Asset, AssetServer},
    audio::AudioSource,
    prelude::{Assets, Handle, Resource},
    render::texture::Image,
};
use rand::seq::SliceRandom;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fmt::{Display, Formatter},
};

lazy_static::lazy_static! {
    static ref AUDIO_MATCHER: Regex = Regex::new(r"_*\d*\.ogg$").expect("Failed to parse audio regex");
    static ref IMAGE_MATCHER: Regex = Regex::new(r"_*\d*\.png$").expect("Failed to parse image regex");
}

const SPARE_PATHS: &[(&str, &str)] = &[("actors/zombie_agile/", "actors/zombie/")];

#[derive(Resource)]
pub struct AssetGroups<T: Asset> {
    groups: HashMap<String, AssetGroup<T>>,
    missing: HashSet<String>,
    matcher: &'static Regex,
}

impl AssetGroups<AudioSource> {
    pub fn new_for_audio() -> Self {
        return Self::new(&AUDIO_MATCHER);
    }
}

impl AssetGroups<Image> {
    pub fn new_for_image() -> Self {
        return Self::new(&IMAGE_MATCHER);
    }
}

impl<T: Asset> AssetGroups<T> {
    pub fn new(matcher: &'static Regex) -> Self {
        return Self {
            groups: HashMap::new(),
            missing: HashSet::new(),
            matcher,
        };
    }

    pub fn index(&mut self, assets: &Assets<T>, asset_server: &AssetServer, shuffle: bool) {
        log::debug!("{}. Indexing...", self);
        self.groups.clear();
        self.missing.clear();

        for asset_id in assets.ids() {
            if let Some(handle) = asset_server.get_id_handle(asset_id) {
                if let Some(path) = handle.path() {
                    let asset_path = path.path().display().to_string().replace('\\', "/");
                    let mut group_path = self.matcher.replace_all(&asset_path, "").into_owned();

                    if let Some('/') = group_path.chars().last() {
                        group_path.pop();
                    }

                    self.groups
                        .entry(group_path)
                        .or_insert_with(AssetGroup::default)
                        .assets
                        .push(handle);
                }
            }
        }

        let prefix = format!("{}", self);
        for (group_path, group) in self.groups.iter_mut() {
            log::debug!(
                "{}. Indexed assets: {} of {}",
                prefix,
                group.assets.len(),
                group_path,
            );

            if shuffle {
                group.shuffle();
            }
        }

        log::debug!("{}. Indexed groups: {}", self, self.groups.len());
    }

    // TODO optimize
    pub fn get_group(&mut self, path: &str) -> Option<&mut AssetGroup<T>> {
        let mut valid_path = None;

        if self.groups.contains_key(path) {
            valid_path = Some(SmartString::from(path));
        } else {
            if self.missing.contains(path) {
                return None;
            }

            for (prefix, replacement) in SPARE_PATHS {
                if path.starts_with(prefix) {
                    let spare_path = path.replace(prefix, replacement);

                    if self.groups.contains_key(&spare_path) {
                        valid_path = Some(SmartString::from(spare_path));
                        break;
                    }
                }
            }
        }

        if let Some(valid_path) = valid_path {
            return self.groups.get_mut(valid_path.as_ref());
        }

        self.missing.insert(path.to_string());
        log::warn!("{}. Group {} not found", self, path);

        return None;
    }
}

impl<T: Asset> Display for AssetGroups<T> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        // TODO: print short name
        return write!(f, "AssetGroups<{}>", std::any::type_name::<T>());
    }
}

pub struct AssetGroup<T: Asset> {
    assets: Vec<Handle<T>>,
    cursor: usize,
}

impl<T: Asset> Default for AssetGroup<T> {
    fn default() -> Self {
        return Self {
            assets: Vec::new(),
            cursor: 0,
        };
    }
}

impl<T: Asset> AssetGroup<T> {
    pub fn get(&self, i: usize) -> Option<Handle<T>> {
        return self.assets.get(i).cloned();
    }

    pub fn get_next(&mut self, shuffle: bool) -> Option<Handle<T>> {
        self.cursor = (self.cursor + 1) % self.assets.len();
        let asset = self.get(self.cursor);

        if shuffle && self.assets.len() > 2 && self.cursor + 1 == self.assets.len() {
            self.shuffle();
        }

        return asset;
    }

    pub fn shuffle(&mut self) {
        self.assets.shuffle(&mut rand::thread_rng());
    }

    pub fn len(&self) -> usize {
        return self.assets.len();
    }
}
