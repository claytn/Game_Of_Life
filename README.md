# GameOfLife
Rust implementation of John Conway's ["Game of Life"](https://bitstorm.org/gameoflife/)
 
# Rules
  ### For a cell that is 'populated':
  1. Each cell with one or no neighbors dies
  2. Each cell with four or more neighbors dies
  3. Each cell with two or three neighbors survives
  
  ### For a space that is 'unpopulated':
  1. Each cell with three neighbors becomes populated.

