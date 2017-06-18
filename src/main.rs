mod grid;
use grid::Grid;

fn main() {
	/* Game of life with a buffered grid approach */
	let gameBoard:Grid = Grid{ currentBuffer: 1, one: [[false; 8]; 8], two: [[false; 8]; 8] };
	loop{
		/* repaint non-existent GUI */
	}
}
