# Game_Of_Life
Rust implementation of John Conway's ["Game of Life"](https://bitstorm.org/gameoflife/)

# Rules
  ### For a cell that is 'populated':
  1. Each cell with one or no neighbors dies
  2. Each cell with four or more neighbors dies
  3. Each cell with two or three neighbors survives

  ### For a space that is 'unpopulated':
  1. Each cell with three neighbors becomes populated.


# Usage
 * A GUI version should be released soon, but until then we shall use files!
  ### Step 1: Install cargo and the nightly rust compiler
  ### Step 2: Make a file (inside project root) that contains an 8x8 grid using the characters '-' and '*'
   #### '*' represents a populated cell and '-' is an unpopulated cell.
   Example:
   ```
   --------
   --------
   ---**---
   ---*----
   --------
   ---***--
   --------
   --------
   ```
  ### Step 3: Inside the project directory run `cargo run <file name> <# of iterations>`
  #### Example: `cargo run example_grid.txt 4`
