#![feature(inclusive_range_syntax)]
#![allow(non_snake_case)]

mod grid;
use grid::Grid;
use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::process;

fn file_as_grid(file_contents:String, grid:&mut [[bool; 8]; 8]) -> Result<(), String>{
	/* transform file contents into an 8x8 bool array */
	for (i, line) in file_contents.lines().enumerate(){
		for (j, c) in line.chars().enumerate(){
			if c != '\n'{
				if c == '*' {
					grid[i][j] = true;
				}
				else if c != '-'{
					return Err(String::from("Invalid characters in file."));
				}
			}
		}
	}
	Ok(())
}

fn grab_file_contents(args:&Vec<String>) -> Result<String, String>{
	let mut f = File::open(&args[1]).expect("Failed to open file provided.");
	let mut file_contents = String::new();
	f.read_to_string(&mut file_contents).expect("Failed reading file to string.");
	Ok(file_contents)
}

fn grid_to_string(grid:&[[bool; 8]; 8]) -> String{
	let mut data = String::new();
	for i in 0..8{
		for j in 0..8{
			if grid[i][j] == true{
				data.push('*');
			}
			else{
				data.push('-');
			}
		}
		data.push('\n');
	}
	data
}

fn main() {
	let args:Vec<String> = env::args().collect();
	if args.len() < 3{
		println!("Usage: cargo run <file> <# of iterations>");
		process::exit(1);
	}

	let file_contents = grab_file_contents(&args).unwrap_or_else(|err|{
			println!("Problem with getting file contents: {}", err);
			process::exit(1);
	});

	// write file contents into readGrid
	let mut read_grid = [[false; 8]; 8];
	file_as_grid(file_contents, &mut read_grid).unwrap_or_else(|err|{
			println!("Error parsing file into grid: {}", err);
			process::exit(1);
	});

	let mut game_board:Grid = Grid::new(read_grid);

	let iterations:u8 = args[2].parse().unwrap_or_else(|err|{
		println!("Error parsing # of iterations: {}", err);
		process::exit(1);
	});

	for _ in 0..iterations{
		game_board.next();
	}

	println!("{}", grid_to_string(&game_board.display_grid()));
}
