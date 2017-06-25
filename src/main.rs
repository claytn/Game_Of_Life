#![allow(non_snake_case)]
#![feature(inclusive_range_syntax)]
mod grid;
use grid::Grid;
use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::process;

fn fileAsGrid(fileContents:String) -> Result<[[bool;8]; 8], String>{
	/* transform file contents into an 8x8 bool array */
	let rows = fileContents.lines();
	if rows.len() != 8{ return Err("Grid in file must be 8x8"); }
	for line in fileContents.lines(){
		if line.trim().len() != 8{ return Err("Grid in file must be 8x8"); }
		for c in fileContents.chars(){
			if c != '\n'{
				if c == '*'{

				}
			}
		}
	}

	Ok([[false;8];8])
}

fn grabFileContents(args:&Vec<String>) -> Result<String, String>{
	if args.len() < 2{
		return Err(String::from("Not enough arguments\nUsage: cargo run <fileName>"));
	}
	let mut f = File::open(&args[1]).expect("Failed to open file provided.");
	let mut fileContents = String::new();
	f.read_to_string(&mut fileContents).expect("Failed reading file to string.");
	Ok(fileContents)
}

fn main() {
	let args:Vec<String> = env::args().collect();
	let fileContents = grabFileContents(&args).unwrap_or_else(|err|{
			println!("Problem with getting file contents: {}", err);
			process::exit(1);
	});
	let mut requestedGrid = fileAsGrid(fileContents).unwrap_or_else(|err|{
			println!("Error parsing file into grid: {}", err);
			process::exit(1);
	});

	let mut game_board:Grid = Grid::new();
	println!("{:?}", game_board.display());
	game_board.next();
	println!("-------------------------------------");
	println!("{:?}", game_board.display());
}
