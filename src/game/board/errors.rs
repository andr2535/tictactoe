use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum BoardGetError {
	ParseIntError(std::num::ParseIntError),
	IdentifierOutOfRange
}
impl Error for BoardGetError { }
impl Display for BoardGetError {
	fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		use BoardGetError::*;
		match self {
			ParseIntError(_err) => write!(fmt, "Invalid number given"),
			IdentifierOutOfRange => write!(fmt, "Too high number given")
		}
	}
}
impl From<std::num::ParseIntError> for BoardGetError {
	fn from(error: std::num::ParseIntError) -> Self {
		BoardGetError::ParseIntError(error)
	}
}

#[derive(Debug)]
pub enum BoardSetError {
	BoardGet(BoardGetError),
	SymbolAlreadySet
}
impl Error for BoardSetError { }
impl Display for BoardSetError {
	fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
		use BoardSetError::*;
		match self {
			BoardGet(err) => write!(fmt, "BoardSetError -> BoardGetError: {}", err),
			SymbolAlreadySet => write!(fmt, "Symbol already set at chosen position")
		}
	}
}
impl From<BoardGetError> for BoardSetError {
	fn from(err: BoardGetError) -> Self {
		BoardSetError::BoardGet(err)
	}
}