pub trait Clock {
    fn now(&self) -> chrono::DateTime<chrono::Utc>;
}

impl Clock for chrono::Utc {
    fn now(&self) -> chrono::DateTime<chrono::Utc> {
        chrono::Utc::now()
    }
}

#[cfg(test)]
#[derive(Clone, Copy, Debug)]
pub struct MockClock {
    now: chrono::DateTime<chrono::Utc>,
}

#[cfg(test)]
impl MockClock {
    pub fn new(now: chrono::DateTime<chrono::Utc>) -> Self {
        Self { now }
    }
}

#[cfg(test)]
impl Clock for MockClock {
    fn now(&self) -> chrono::DateTime<chrono::Utc> {
        self.now
    }
}
