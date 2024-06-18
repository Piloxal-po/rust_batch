use std::fmt::Display;

use chrono::{DateTime, Local, TimeDelta};

pub struct TimeKeeper {
    start_time: Option<DateTime<Local>>,
    end_time: Option<DateTime<Local>>,
    last_update_time: Option<DateTime<Local>>,
}

impl TimeKeeper {
    pub fn new() -> Self {
        TimeKeeper {
            start_time: None,
            end_time: None,
            last_update_time: None,
        }
    }

    pub fn start(&mut self) -> &mut TimeKeeper {
        let now = Local::now();
        self.start_time = Some(now);
        self.update(now)
    }

    pub fn update(&mut self, instant: DateTime<Local>) -> &mut TimeKeeper {
        self.last_update_time = Some(instant);
        self
    }

    pub fn end(&mut self) -> &mut TimeKeeper {
        let now = Local::now();
        self.end_time = Some(now);
        self.update(now)
    }

    pub fn run_time(&self) -> TimeDelta {
        let now = Local::now();
        self.end_time
            .unwrap_or(now)
            .signed_duration_since(self.start_time.unwrap_or(now))
    }
}

impl Clone for TimeKeeper {
    fn clone(&self) -> Self {
        TimeKeeper {
            start_time: self.start_time,
            end_time: self.end_time,
            last_update_time: self.last_update_time,
        }
    }
}

impl Copy for TimeKeeper {}

impl Display for TimeKeeper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "start : {:?}, end : {:?}, run_time : {}",
            self.start_time,
            self.end_time,
            self.run_time()
        )
    }
}
