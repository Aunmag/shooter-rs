use crate::{
    command::ActorSet,
    component::ActorConfig,
    model::TransformLite,
    plugin::{bot::ActorBotSet, WeaponConfig, WeaponSet},
};
use bevy::{
    ecs::system::Command,
    prelude::{Commands, World},
};
use rand::{seq::SliceRandom, Rng};

const CHANCE_INCREASED: f32 = 3.0;
const CHANCE_DECREASED: f32 = 0.2;

pub struct Wave {
    pub size: u8,
    spawned: u8,
    presets: Vec<Spawn>,
}

impl Wave {
    pub fn spawn<R: Rng>(
        &mut self,
        transform: TransformLite,
        commands: &mut Commands,
        rng: &mut R,
    ) {
        self.spawned = self.spawned.saturating_add(1);

        let Ok(preset) = self.presets.choose_weighted(rng, |s| s.weight).cloned() else {
            return;
        };

        commands.add(move |world: &mut World| {
            let entity = world.spawn_empty().id();

            ActorSet {
                entity,
                config: preset.config,
                transform,
            }
            .apply(world);

            ActorBotSet { entity }.apply(world);

            if let Some(weapon) = preset.weapon {
                WeaponSet {
                    entity,
                    weapon: Some(weapon),
                }
                .apply(world);
            }
        });
    }

    // TODO: rename
    pub fn is_complete(&self) -> bool {
        return self.spawned >= self.size;
    }
}

pub struct WaveSeries {
    waves: Vec<Wave>,
    index: u8,
}

impl WaveSeries {
    pub fn new() -> Self {
        let waves = vec![
            // 1. Initial
            Wave {
                size: 5,
                spawned: 0,
                presets: vec![Spawn::ZOMBIE_DEFAULT.clone()],
            },
            // 2. Some agile
            Wave {
                size: 20,
                spawned: 0,
                presets: vec![Spawn::ZOMBIE_DEFAULT.clone(), Spawn::ZOMBIE_AGILE.clone()],
            },
            // 3. Some pistols
            Wave {
                size: 40,
                spawned: 0,
                presets: vec![
                    Spawn::ZOMBIE_DEFAULT.clone(),
                    Spawn::ZOMBIE_AGILE.less(),
                    Spawn::ZOMBIE_WITH_PISTOL.clone(),
                ],
            },
            // 4. More agile
            Wave {
                size: 60,
                spawned: 0,
                presets: vec![
                    Spawn::ZOMBIE_DEFAULT.clone(),
                    Spawn::ZOMBIE_AGILE.more(),
                    Spawn::ZOMBIE_WITH_PISTOL.less(),
                ],
            },
            // 5. More armed
            Wave {
                size: 80,
                spawned: 0,
                presets: vec![
                    Spawn::ZOMBIE_DEFAULT.clone(),
                    Spawn::ZOMBIE_AGILE.clone(),
                    Spawn::ZOMBIE_WITH_PISTOL.more(),
                ],
            },
            // 6. Some rifles
            Wave {
                size: 100,
                spawned: 0,
                // TODO: spawn some humans at the end
                presets: vec![
                    Spawn::ZOMBIE_DEFAULT.clone(),
                    Spawn::ZOMBIE_AGILE.clone(),
                    Spawn::ZOMBIE_WITH_PISTOL.clone(),
                    Spawn::ZOMBIE_WITH_RIFLE.clone(),
                ],
            },
        ];

        return Self { waves, index: 0 };
    }

    pub fn next(&mut self) {
        self.index = self.index.saturating_add(1);
    }

    pub fn get_current(&mut self) -> Option<&mut Wave> {
        return self.waves.get_mut(usize::from(self.index));
    }

    pub fn get_wave_number(&self) -> u8 {
        return self.index.saturating_add(1);
    }

    pub fn get_waves_count(&self) -> usize {
        return self.waves.len();
    }

    pub fn is_final(&self) -> bool {
        return usize::from(self.index.saturating_add(1)) == self.waves.len();
    }
}

#[derive(Clone)]
pub struct Spawn {
    config: &'static ActorConfig,
    weapon: Option<&'static WeaponConfig>,
    weight: f32,
}

impl Spawn {
    const ZOMBIE_DEFAULT: Self = Self {
        config: &ActorConfig::ZOMBIE,
        weapon: None,
        weight: 1.0,
    };

    const ZOMBIE_AGILE: Self = Self {
        config: &ActorConfig::ZOMBIE_AGILE,
        weapon: None,
        weight: 0.1, // TODO: tweak
    };

    const ZOMBIE_WITH_PISTOL: Self = Self {
        config: &ActorConfig::ZOMBIE,
        weapon: Some(&WeaponConfig::PM),
        weight: 0.3, // TODO: tweak
    };

    const ZOMBIE_WITH_RIFLE: Self = Self {
        config: &ActorConfig::ZOMBIE,
        weapon: Some(&WeaponConfig::AKS_74U),
        weight: 0.1, // TODO: tweak
    };

    pub fn less(&self) -> Self {
        return self.with_weight(CHANCE_DECREASED);
    }

    pub fn more(&self) -> Self {
        return self.with_weight(CHANCE_INCREASED);
    }

    pub fn with_weight(&self, weight: f32) -> Self {
        return Self {
            config: self.config,
            weapon: self.weapon,
            weight: self.weight * weight,
        };
    }
}
