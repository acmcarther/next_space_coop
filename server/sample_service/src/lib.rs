extern crate sample_service_proto;
extern crate service_proto;
extern crate service;
extern crate specs;
extern crate protobuf;

use service::Service;
use service_proto::service::DependencyConfig;
use service_proto::service::ServiceConfig;
use specs::System;
use specs::RunArg;
use sample_service_proto::sample_service::SampleServiceState;
use protobuf::Message;

pub struct SampleService {
  state: SampleServiceState,
}

impl SampleService {
  pub fn new() -> SampleService {
    SampleService {
      state: SampleServiceState::new()
    }
  }

  pub fn default_config() -> ServiceConfig {
    let mut svc_config = ServiceConfig::new();
    let mut dep_config = DependencyConfig::new();

    svc_config.set_name("SampleService".to_owned());
    svc_config.set_dependency_config(dep_config);

    svc_config
  }
}

impl System<u64> for SampleService {
  fn run(&mut self, _: RunArg, _: u64) {

  }
}

impl Service for SampleService {
  fn use_state(&mut self, state: &[u8]) {
    self.state = protobuf::parse_from_bytes(state).unwrap()
  }

  fn dump_state(self) -> Vec<u8> {
    self.state.write_to_bytes().unwrap()
  }
}
