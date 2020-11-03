use super::symbol::Symbol;
#[derive(Debug)]
pub struct Player {
	pub name: String,
	pub symbol: Symbol,
	pub wins: usize
}