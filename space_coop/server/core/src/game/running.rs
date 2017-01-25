
use ::network::Network;
use state_proto::state::NetworkConfig;
use state_proto::state::State;
use state_proto::state::Time;
use state_proto::state::Time_TimeMode;
use super::TransientState;
use time::PreciseTime;

pub struct RunningGame {
  state: State,
  transient: TransientState,
  last_run_time: PreciseTime,
}

impl RunningGame {
  pub fn new(state: State, transient: TransientState) -> RunningGame {
    // TODO(acmcarther): Disassemble the state repr here for use in ECS
    RunningGame {
      state: state,
      transient: transient,
      last_run_time: PreciseTime::now(),
    }
  }

  pub fn run(&mut self) {
    let now = PreciseTime::now();

    let delta = self.last_run_time.to(now.clone());

    self.last_run_time = now;
    let microsecond_delta = delta.num_microseconds()
      .expect("time between runs was way too long (over 280k years!)");
    let next_timestamp = self.state.get_time().get_timestamp().clone() + microsecond_delta;
    self.state.mut_time().set_timestamp(next_timestamp);
  }

  pub fn build_state(&self) -> State {
    self.state.clone()
  }

  pub fn take_transient_state(self) -> TransientState {
    self.transient
  }
}
