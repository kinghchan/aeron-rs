pub struct NanoClock;

impl NanoClock {
    pub fn instance() -> Self {
        NanoClock
    }

    pub fn nano_time(&self) -> u64 {
        std::time::Instant::now().elapsed().as_nanos() as u64
    }
}