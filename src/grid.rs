/* Grid definition that holds both grids; one used as a buffer while other is updated */
struct Grid{
        buffer: u8,
        one: [[bool; 8]; 8],
        two: [[bool; 8]; 8]
}

impl Grid{
        fn switch_grid(&self){
                self.currentBuffer = if self.currentBuffer == 1{ 2 } else{ 1 };
        }

        fn valid_cell(x:i32, y:i32)->bool{
                if x >= 0 && x < 8 && y < 8 && y >= 0{
                        return true;
                }
                return false;
        }

        fn num_neighbors(grid:[[bool;8];8], x:u32, y:u32) -> u32{
                let mut count:u8 = 0;
                /* check each square touching current square */
        }

        fn neighbors(&self, x:u32, y:u32) -> u32{
                if self.buffer == 1{
                        return Grid::num_neighbors(&self.one, x, y);
                }
                return Grid::num_neighbors(&self.two, x, y);
        }
}
