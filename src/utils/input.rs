use std::sync::atomic::AtomicI16;
use std::sync::atomic::Ordering;

// TODO: Try make an object with methods for it
// TODO: Make sure it's a good way to tack mouse movement with minimal delay
// TODO: Maybe it's better to store a decimal value, at least I've seen integers only so far
static MOUSE_DELTA: AtomicI16 = AtomicI16::new(0);

pub fn reset_mouse_delta() {
    MOUSE_DELTA.store(0, Ordering::Relaxed);
}

pub fn add_mouse_delta(value: i16) {
    // TODO: Make sure it's a proper way to do an atomic saturating addition
    MOUSE_DELTA.store(
        MOUSE_DELTA.load(Ordering::Relaxed).saturating_add(value),
        Ordering::Relaxed,
    );
}

pub fn take_mouse_delta() -> i16 {
    return MOUSE_DELTA.swap(0, Ordering::Relaxed);
}
