use bevy::platform::collections::HashMap;
use std::{
    sync::{Mutex, OnceLock},
    time::{Duration, Instant},
};

macro_rules! bench {
    () => {
        #[cfg(feature = "bench")]
        let _bench_guard = crate::util::bench::BenchGuard::new(concat!(file!(), ":", line!()));
    };
}

pub(crate) use bench;

const REPORT_INTERVAL: Duration = Duration::from_secs(5);

static GLOBAL: OnceLock<Mutex<Benches>> = OnceLock::new();

struct Benches {
    benches: HashMap<&'static str, Duration>,
    next_report: Instant,
}

impl Benches {
    fn new() -> Self {
        return Self {
            benches: HashMap::new(),
            next_report: Instant::now() + REPORT_INTERVAL,
        };
    }

    fn report(&self) {
        let mut report = Vec::with_capacity(self.benches.len());
        let mut report_string = "## Bench report".to_string();
        let mut total_duration = Duration::ZERO;

        for (name, duration) in self.benches.iter() {
            total_duration += *duration;
            report.push((name, duration));
        }

        report.sort_by(|a, b| a.1.cmp(b.1).reverse());

        for (name, duration) in report {
            report_string.push_str(&format!(
                "\n- {:.1}% {:.3}s {}",
                duration.as_secs_f32() / total_duration.as_secs_f32() * 100.0,
                duration.as_secs_f32(),
                name
            ));
        }

        log::info!("{}", report_string);
    }
}

pub struct BenchGuard {
    name: &'static str,
    started: Instant,
}

impl BenchGuard {
    pub fn new(name: &'static str) -> Self {
        return Self {
            name,
            started: Instant::now(),
        };
    }
}

impl Drop for BenchGuard {
    fn drop(&mut self) {
        let now = Instant::now();
        let duration = now - self.started;

        let Ok(mut benches) = GLOBAL.get_or_init(|| Mutex::new(Benches::new())).lock() else {
            return;
        };

        *benches.benches.entry(self.name).or_insert(Duration::ZERO) += duration;

        if benches.next_report <= now {
            benches.next_report = now + REPORT_INTERVAL;
            benches.report();
        }
    }
}
