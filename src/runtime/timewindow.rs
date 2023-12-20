use std::time::{Instant, Duration};

pub struct TimeWindow {
    start_time: Option<Instant>,
    total_duration: Duration,
    count: u32,
}

impl TimeWindow {
    pub fn new() -> Self {
        TimeWindow {
            start_time: None,
            total_duration: Duration::from_secs(0),
            count: 0,
        }
    }
    
    pub fn open_window(&mut self) {
        if self.start_time.is_none() {
            self.start_time = Some(Instant::now());
        }
    }

    pub fn close_window(&mut self) {
        if let Some(start_time) = self.start_time {
            let duration = start_time.elapsed();
            self.total_duration += duration;
            self.count += 1;
            self.start_time = None;
        }
    }

    pub fn get_average_duration(&self) -> Option<Duration> {
        if self.count > 0 {
            Some(self.total_duration / self.count)
        } else {
            None
        }
    }
}
