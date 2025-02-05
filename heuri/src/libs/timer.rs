
/* Timer {{{ */
struct Timer {
    instant: std::time::Instant,
    timeout_ms: u128,
}

#[allow(dead_code)]
impl Timer {
    fn new(timeout_ms: u128) -> Self {
        Self {
            instant: std::time::Instant::now(),
            timeout_ms,
        }
    }

    fn elapsed(&self) -> u128 {
        self.instant.elapsed().as_millis()
    }

    fn is_timeout(&self) -> bool {
        self.elapsed() >= self.timeout_ms
    }

    fn reset(&mut self) {
        self.instant = std::time::Instant::now();
    }

    fn set_timeout(&mut self, timeout_ms: u128) {
        self.timeout_ms = timeout_ms;
    }
}
/* }}} */
