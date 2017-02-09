use std::time::SystemTime;

pub trait Clock: Send {
  fn now(&self) -> SystemTime;
}

pub struct SystemClock;

impl SystemClock {
  pub fn new() -> SystemClock {
    SystemClock
  }
}

impl Clock for SystemClock {
  fn now(&self) -> SystemTime {
    SystemTime::now()
  }
}

pub struct MockClock {
  _now: SystemTime,
}

impl MockClock {
  pub fn new() -> MockClock {
    MockClock {
      _now: SystemTime::now(),
    }
  }

  pub fn set_time(&mut self, time: SystemTime) {
    self._now = time
  }
}

impl Clock for MockClock {
  fn now(&self) -> SystemTime {
    self._now.clone()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::time::SystemTime;

  #[test]
  fn mock_returns_what_is_set() {
    let time = SystemTime::now();
    let mut mock_clock = MockClock::new();

    mock_clock.set_time(time.clone());
    assert_eq!(mock_clock.now(), time);
  }
}
