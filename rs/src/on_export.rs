use std::thread::sleep;
use std::time::Duration;

const SLEEP_HEURISTIC_MILLIS: u64 = 50;

pub fn wait() {
    sleep(Duration::from_millis(SLEEP_HEURISTIC_MILLIS))
}
