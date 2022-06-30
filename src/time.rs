use std::time::SystemTime;

pub struct Time;

impl Time {
    pub fn now() -> u128 {
        SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis()
    }

    pub fn stamp() -> u64 {
        SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
    }
}
