use state_proto::state::Game;
use game_proto::game::GameMode;
use itertools::Itertools;

// TODO(acmcarther): Remove lint killer
#[allow(dead_code)]
pub struct Gameplay<'a> {
  game_state: &'a mut Game,
  microsecond_delta: i64
}

impl <'a> Gameplay<'a> {
  pub fn new(game_state: &mut Game, microsecond_delta: i64) -> Gameplay {
    Gameplay {
      game_state: game_state,
      microsecond_delta: microsecond_delta
    }
  }

  pub fn tick(self) {
    // No gameplay yet ~> Back to the lobby with you!
    self.game_state.set_mode(GameMode::LOBBY);
    self.game_state.mut_player_data().iter_mut().foreach(|p| p.clear_match_data());
  }
}
