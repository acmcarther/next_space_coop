use state_proto::state::Game;
use game_proto::game::GameMode;
use state_proto::state::PlayerData;
use game_proto::game::MatchData;
use game_proto::game::MatchData_Position;
use itertools::Itertools;

/** The lobby is where players wait till theres enough ready people to play! */
pub struct Lobby<'a> {
  game_state: &'a mut Game
}

impl <'a> Lobby<'a> {
  pub fn new(game_state: &mut Game) -> Lobby {
    Lobby {
      game_state: game_state
    }
  }

  pub fn tick(mut self) {
    let ready_player_idxs = self.game_state.get_player_data().iter()
      .enumerate()
      .filter(|&(_, p)| p.get_active() && p.get_ready())
      .map(|(idx, _)| idx)
      .collect::<Vec<usize>>();

    if ready_player_idxs.len() > 2 {
      self.game_state.set_mode(GameMode::PLAYING);

      let idx_1 = ready_player_idxs.get(0).unwrap();
      let idx_2 = ready_player_idxs.get(1).unwrap();
      let mut player_data = self.game_state.mut_player_data();

      player_data.iter_mut().foreach(|p| p.clear_match_data());

      Lobby::init_match_for_player(player_data.get_mut(*idx_1).unwrap(), 1.0 /* x pos */);
      Lobby::init_match_for_player(player_data.get_mut(*idx_2).unwrap(), -1.0 /* x pos */);
      player_data.iter_mut().foreach(|p| p.clear_ready());
    }
  }

  fn init_match_for_player(player_data: &mut PlayerData, x_pos: f32) {
    let mut player_match_data = MatchData::new();
    let mut position = MatchData_Position::new();
    position.set_x(x_pos);
    position.set_y(0.0);
    position.set_z(0.0);
    player_match_data.set_position(position);
    player_data.set_match_data(player_match_data);
  }
}
