use std::thread::sleep;
use std::time::Duration;

const SLEEP_HEURISTIC_MILLIS: u64 = 50;

/// wait ~50ms after exporting this pin.
/// if you set_direction *immediately* after
/// entering this closure, without your
/// pin having been exported on a *previous*
/// run, you'll crash.
pub fn wait() {
    sleep(Duration::from_millis(SLEEP_HEURISTIC_MILLIS))
}
