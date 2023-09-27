use bevy::ecs::component::Component;
use rand::{thread_rng, Rng};
use std::{collections::HashMap, time::Duration};
use strum::{EnumIter, IntoEnumIterator};

const INTERVAL_MIN: Duration = Duration::from_millis(240);
const INTERVAL_MID: Duration = Duration::from_secs(10);
const INTERVAL_COMMENT_COMBAT: Duration = Duration::from_secs(20);

const DELAY_FAST: Duration = Duration::from_millis(100);
const DELAY_MID: Duration = Duration::from_millis(600);
const DELAY_SLOW: Duration = Duration::from_millis(1000);

// TODO: don't play if already playing

#[derive(Component)]
pub struct Voice {
    sound: Option<VoiceSound>,
    queued: Duration,
    delay: Duration,
    next: HashMap<VoiceSound, Duration>,
    next_any: Duration,
}

impl Voice {
    pub fn new(time: Duration) -> Self {
        let mut voice = Self {
            sound: None,
            queued: Duration::ZERO,
            delay: Duration::ZERO,
            next: HashMap::new(),
            next_any: Duration::ZERO,
        };

        // add first intervals
        for sound in VoiceSound::iter() {
            if let VoiceSound::Start(..) = sound {
                continue;
            }

            voice.next.insert(sound, time + sound.generate_interval());
        }

        return voice;
    }

    // TODO: test
    pub fn queue(&mut self, sound: VoiceSound, time: Duration) {
        let check_time = VoiceSound::Death != sound;

        if let Some(queued) = self.sound {
            if sound.priority() > queued.priority() {
                self.set_sound(sound, time);
                return;
            }
        }

        if check_time && time < self.next_any {
            return;
        }

        if check_time && time
            < self
                .next
                .get(&sound.zero())
                .copied()
                .unwrap_or(Duration::ZERO)
        {
            return;
        }

        self.set_sound(sound, time);
    }

    fn set_sound(&mut self, sound: VoiceSound, time: Duration) {
        self.sound = Some(sound);
        self.queued = time;
        self.delay = sound.delay();
        self.next
            .insert(sound.zero(), time + sound.generate_interval());
        self.next_any = time + INTERVAL_MIN;
    }

    // TODO: maybe return misc from here
    pub fn pop_queued(&mut self, time: Duration) -> Option<VoiceSound> {
        if self.queued + self.delay < time {
            if let Some(sound) = self.sound.take() {
                return Some(sound);
            }
        }

        return None;
    }
}

// TODO: move to model
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum VoiceSound {
    Death,
    Pain,
    Start(bool),
    Success,
    Fear, // TODO: impl
    ArmBest,
    ArmBetter,
    ArmShotgun,
    ArmWorse,
    ArmSimilar,
    Kill,
    Hit,
    Misc,
    Reload(bool),
}

impl VoiceSound {
    // TODO: simplify
    pub fn name(&self) -> &'static str {
        return match self {
            Self::ArmBest => "arm_best",
            Self::ArmBetter => "arm_better",
            Self::ArmShotgun => "arm_shotgun",
            Self::ArmSimilar => "arm_similar",
            Self::ArmWorse => "arm_worse",
            Self::Death => "death",
            Self::Fear => "fear",
            Self::Hit => "hit",
            Self::Kill => "kill",
            Self::Misc => "misc",
            Self::Pain => "pain",
            Self::Reload(false) => "reload_0",
            Self::Reload(true) => "reload_1",
            Self::Start(false) => "start_0",
            Self::Start(true) => "start_1",
            Self::Success => "success",
        };
    }

    pub fn delay(&self) -> Duration {
        return match self {
            Self::Death => Duration::ZERO, // play instantly, before entity will be despawned
            Self::Pain
            | Self::Reload(..)
            | Self::ArmWorse
            | Self::ArmSimilar
            | Self::ArmBetter
            | Self::ArmShotgun
            | Self::ArmBest => DELAY_FAST,
            Self::Hit | Self::Kill | Self::Fear => DELAY_MID,
            Self::Start(..) | Self::Success | Self::Misc => DELAY_SLOW,
        };
    }

    pub fn generate_interval(&self) -> Duration {
        let interval = match self {
            Self::ArmBest
            | Self::ArmBetter
            | Self::ArmShotgun
            | Self::ArmSimilar
            | Self::ArmWorse => INTERVAL_MID,
            Self::Fear | Self::Hit | Self::Kill | Self::Reload(..) | Self::Misc => {
                INTERVAL_COMMENT_COMBAT
            }
            Self::Death | Self::Pain | Self::Start(..) | Self::Success => INTERVAL_MIN,
        };

        return randomize_duration(interval);
    }

    // TODO: rename
    pub fn zero(self) -> Self {
        return match self {
            Self::Start(..) => Self::Start(false),
            Self::Reload(..) => Self::Reload(false),
            sound => sound,
        };
    }

    // TODO: find a way to optimize
    pub fn priority(&self) -> usize {
        let a = self.zero(); // TODO: rename

        for (i, sound) in Self::iter().rev().enumerate() {
            if sound == a {
                return i;
            }
        }

        return 0;
    }
}

fn randomize_duration(duration: Duration) -> Duration {
    let part = duration / 4;
    let min = duration.saturating_sub(part);
    let max = duration.saturating_add(part);

    if min == max {
        return min;
    } else {
        return thread_rng().gen_range(min..max);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order() {
        assert_eq!(VoiceSound::Death.priority(), 13);
        assert_eq!(VoiceSound::Pain.priority(), 12);
        assert_eq!(VoiceSound::Misc.priority(), 1);
    }
}
