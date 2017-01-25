extern crate event_bus_proto;

use event_bus_proto::event_bus::Event;
use event_bus_proto::event_bus::Frame;
use event_bus_proto::event_bus::Index;
use event_bus_proto::event_bus::Snapshot;

pub struct EventBus(Snapshot);

impl EventBus {
  pub fn new() -> EventBus {
    EventBus(Snapshot::new())
  }

  pub fn load_from_snapshot(snapshot: Snapshot) -> EventBus {
    EventBus(snapshot)
  }
}
