mod errors;
use errors::*;
use super::symbol::Symbol;
use std::ops::Add;
#[derive(Debug, Copy, Clone)]
struct BoardVector {
	x: isize,
	y: isize
}
impl Add for BoardVector {
	type Output = BoardVector;
	fn add(self, other: BoardVector) -> BoardVector {
		BoardVector{x: self.x + other.x, y: self.y + other.y}
	}
}

#[derive(Debug)]
pub struct Board {
	height: usize,
	width: usize,
	field_array: Vec<Symbol>
}
impl Board {
	pub fn new(height: usize, width: usize) -> Board {
		Board{height, width, field_array: vec![Symbol::Empty; height*width]}
	}

	fn get_index_from_coordinates(&self, vector: &BoardVector) -> Result<usize, BoardGetError> {
		if vector.x < 0 || vector.x >= self.width as isize || vector.y < 0 || vector.y >= self.height as isize {
			return Err(BoardGetError::IdentifierOutOfRange);
		}
		let index = self.width.wrapping_mul(vector.y as usize).wrapping_add(vector.x as usize);
		if index >= self.field_array.len() {
			Err(BoardGetError::IdentifierOutOfRange)
		}
		else {
			Ok(index)
		}
	}

	/// Returns coordinates of identifier by (x, y) coordinates in a tuple.
	fn get_coordinates_from_identifer(&self, identifier: &str) -> Result<BoardVector, BoardGetError> {
		let identifier = usize::from_str_radix(identifier, 10)?;
		if identifier >= self.field_array.len() {return Err(BoardGetError::IdentifierOutOfRange); }

		// Find proper coordinates.
		let x = identifier % self.width;
		let y = identifier / self.width;
		Ok(BoardVector{x: x as isize,y: y as isize})
	}

	/// Counts the given direction and the reversed direction + 1
	fn symbol_counter(&self, coordinates: BoardVector, direction: BoardVector, symbol: Symbol) -> usize {
		self.symbol_counter_recursive(coordinates, direction, symbol) +
		self.symbol_counter_recursive(coordinates, BoardVector{x: direction.x * -1, y: direction.y * -1}, symbol) + 1
	}
	fn symbol_counter_recursive(&self, coordinates: BoardVector, direction: BoardVector, symbol: Symbol) -> usize {
		let cur_coords = coordinates + direction;
		let index = if let Ok(index) = self.get_index_from_coordinates(&cur_coords) {
			index
		} else {
			// If coordinates are outside bounds, we have reached the end of the line.
			return 0;
		};

		if let Some(coords_symbol) = self.field_array.get(index) {
			if *coords_symbol == symbol {
				1 + self.symbol_counter_recursive(cur_coords, direction, symbol)
			}
			else {0}
		}
		else {0}
	}
	/// Returns the longest line length which the symbol is placed in.
	pub fn set_symbol_identifier(&mut self, identifier: &str, symbol: Symbol) -> Result<usize, BoardSetError> {
		let position = self.get_coordinates_from_identifer(identifier)?;

		let index = self.get_index_from_coordinates(&position)?;
		let symbol_ref = &mut self.field_array[index];

		if *symbol_ref == Symbol::Empty {
			*symbol_ref = symbol;
			// We define all the directions that we can get a line in, and we pick the longest continual line.
			// We only define one direction because the reversed direction will be calculated in the self.symbol_counter function.
			let max = [BoardVector{x: 1,  y: 1},BoardVector{x: 1,  y: -1},BoardVector{x: 1,  y: 0},BoardVector{x: 0,  y: 1}]
			.iter().fold(0, |max, vector| {
				let new = self.symbol_counter(position, *vector, symbol);
				if new > max {new} else {max}
			});
			Ok(max)
		}
		else {
			Err(BoardSetError::SymbolAlreadySet)
		}
	}
}
impl std::fmt::Display for Board {
	fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		let number_width = format!("{}", self.field_array.len()-1).len();

		let mut string = String::new();

		for row_number in 0..self.height {
			string.push('|');
			for column_number in 0..self.width {
				if let Symbol::Set(character) = self.field_array[self.get_index_from_coordinates(&BoardVector{x: column_number as isize, y: row_number as isize}).unwrap()] {
					string.push_str(&format!("{:width$}|", character, width=number_width));
				}
				else {
					string.push_str(&format!("{:0width$}|", (self.width*row_number)+column_number, width=number_width));
				}
			}
			string.push_str("\n");
		}

		write!(fmt, "{}", string)
	}
}