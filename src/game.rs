mod board;
mod player;
pub mod symbol;
use board::Board;
use player::Player;
use symbol::Symbol;


#[derive(Debug)]
pub struct Game {
	board: Board,
	players: Vec<Player>,
	win_length: usize,
	cur_player: usize
}
impl Game {
	pub fn new() -> Game {
		let mut players = Vec::with_capacity(2);

		println!("Enter player X name: ");
		let player_name = Game::get_user_input();
		players.push(Player{name: player_name, symbol: Symbol::Set('X'), wins: 0});

		println!("Enter player O name: ");
		let player_name = Game::get_user_input();
		players.push(Player{name: player_name, symbol: Symbol::Set('O'), wins: 0});

		println!("Enter board height: ");
		let height = Game::get_user_input();
		let height = usize::from_str_radix(height.as_ref(), 10).unwrap();

		println!("Enter board width: ");
		let width = Game::get_user_input();
		let width = usize::from_str_radix(width.as_ref(), 10).unwrap();

		println!("Enter winning length: ");
		let win_length = Game::get_user_input();
		let win_length = usize::from_str_radix(win_length.as_ref(), 10).unwrap();

		Game{board: Board::new(height, width), players, win_length, cur_player: 0}
	}

	pub fn start(&mut self) {
		loop {
			println!("{}", self.board);
			println!("Where does {} want to place a piece: ", self.players[self.cur_player].name);

			let identifier = Game::get_user_input();
			match self.board.set_symbol_identifier(identifier.as_ref(), self.players[self.cur_player].symbol) {
				Ok(longest_line_length) => {
					if longest_line_length >= self.win_length {
						println!("{}\nPlayer {} won the game!", self.board, self.players[self.cur_player].name);
						break;
					}
					else {
						self.cur_player += 1;
						if self.cur_player == self.players.len() {self.cur_player = 0;}
					}
				},
				Err(err) => {
					println!("Cannot set piece at position: {}", err);
				}
			}
		}
	}

	fn get_user_input() -> String {
		let stdin = std::io::stdin();
		let mut player_name = String::new();
		stdin.read_line(&mut player_name).unwrap();
		player_name.pop();
		player_name
	}
}