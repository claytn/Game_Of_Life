#![allow(non_snake_case)]
#![feature(inclusive_range_syntax)]

mod grid;
use grid::Grid;

fn main() {
	/* Game of life with a buffered grid approach */
	let mut game_board:Grid = Grid::new();
	println!("{:?}", game_board.display());
	game_board.next();
	println!("-------------------------------------");
	println!("{:?}", game_board.display());
}
