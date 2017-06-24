#![allow(non_snake_case)]
mod grid;
use grid::Grid;

fn main() {
	/* Game of life with a buffered grid approach */
	let mut game_board:Grid = Grid::new();
	game_board.next();

	println!("{:?}", game_board.one);
	println!("-------------------------------------");
	println!("{:?}", game_board.two);

}
