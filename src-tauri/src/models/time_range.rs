pub struct TimeRange {
    start_timestamp: u64,
    end_timestamp: u64,
}

impl TimeRange {
    pub fn start(&self) -> &u64 {
        &self.start_timestamp
    }

    pub fn end(&self) -> &u64 {
        &self.end_timestamp
    }
}