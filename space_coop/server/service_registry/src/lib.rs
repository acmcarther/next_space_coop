extern crate service_proto;
extern crate service;
extern crate specs;
extern crate sample_service;
extern crate itertools;

use service_proto::service::ServiceConfig;
use service_proto::service::RunningService;
use sample_service::SampleService;
use service::Service;
use itertools::Itertools;

pub fn from_scratch() -> Vec<(ServiceConfig, Box<Service>)> {
  vec![(SampleService::default_config(), Box::new(SampleService::new()))]
}

pub fn reload_from_state(running_services: Vec<RunningService>) -> Vec<(ServiceConfig, Box<Service>)> {
  let mut svcs_from_scratch = from_scratch();

  running_services.into_iter()
    .map(extract_config_and_state)
    .foreach(|(config, state)| integrate_state_into_services(config, state, &mut svcs_from_scratch));

  svcs_from_scratch
}

fn extract_config_and_state(mut svc: RunningService) -> (ServiceConfig, Vec<u8>) {
  (svc.take_config(), svc.take_state())
}

// Scan service list and apply state to service if found
fn integrate_state_into_services(
    config: ServiceConfig,
    state: Vec<u8>,
    services: &mut Vec<(ServiceConfig, Box<Service>)>) {
  services.iter_mut().filter(|&&mut (ref new_config, _)| {
    new_config.get_name() == config.get_name()
  }).foreach(|&mut (_, ref mut service)| {
    service.use_state(&state);
  });
}
