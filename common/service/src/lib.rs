extern crate specs;
extern crate service_proto;

// This will not be present unless the "parallel" feature is enabled in specs
// TODO(acmcarther): Fix cargo2bazel to properly include the default features, and let users
// override feature specification
use specs::System;
use service_proto::service::ServiceConfig;

pub trait Service : System<u64> {
  fn use_state(&mut self, state: &[u8]);
  fn dump_state(self) -> Vec<u8>;
}
