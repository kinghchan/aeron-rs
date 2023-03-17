pub struct SystemNanoClock;

impl SystemNanoClock {
    pub fn instance() -> Self {
        SystemNanoClock
    }

    pub fn nano_time(&self) -> u64 {
        std::time::Instant::now().elapsed().as_nanos() as u64
    }
}