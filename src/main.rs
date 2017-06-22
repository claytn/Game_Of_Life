mod grid;
use grid::Grid;

fn main() {
	/* Game of life with a buffered grid approach */
	let gameBoard:Grid = Grid::new();
	gameBoard.next();
}
