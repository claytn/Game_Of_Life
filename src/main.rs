#![allow(non_snake_case)]
#![feature(inclusive_range_syntax)]
mod grid;
use grid::Grid;
use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::process;

fn fileAsGrid(fileContents:String, myGrid:&mut [[bool; 8]; 8]) -> Result<(), String>{
	/* transform file contents into an 8x8 bool array */
	for (i, line) in fileContents.lines().enumerate(){
		for (j, c) in line.chars().enumerate(){
			if c != '\n'{
				if c == '*' {
					myGrid[i][j] = true;
				}
				else if c != '-'{
					return Err(String::from("Invalid characters in file."));
				}
			}
		}
	}
	Ok(())
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

fn gridToString(grid:&[[bool; 8]; 8]) -> String{
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
	let fileContents = grabFileContents(&args).unwrap_or_else(|err|{
			println!("Problem with getting file contents: {}", err);
			process::exit(1);
	});
	let mut readGrid = [[false; 8]; 8];
	let mut requestedGrid = fileAsGrid(fileContents, &mut readGrid).unwrap_or_else(|err|{
			println!("Error parsing file into grid: {}", err);
			process::exit(1);
	});

	let mut game_board:Grid = Grid::new(readGrid.clone());
	if args.len() < 2{
		println!("Usage: cargo run <file> <# of iterations>");
		process::exit(1);
	}

	let iterations = args[2].parse().unwrap();
	for i in 0..iterations{
		game_board.next();
	}

	let mut output = File::create("results.txt").expect("Failed to create output file");
	output.write_all(gridToString(&game_board.display()).as_bytes()).expect("Failed to write to output file.");

}
