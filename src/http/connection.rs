pub struct Connection {
    sent: usize,
}

impl Connection {
    pub fn new() -> Self {
        Self { sent: 0 }
    }

    pub fn send(&mut self, s: &str) {
        // actually send it
        self.sent += s.len();
    }

    pub fn sent(&self) -> usize {
        self.sent
    }
}
