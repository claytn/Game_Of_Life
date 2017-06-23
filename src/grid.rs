/* Grid definition that holds both grids; one used as a buffer while other is updated */

pub struct Grid{
    currentBuffer: u8,
    display: u8,
    one: [[bool; 8]; 8],
    two: [[bool; 8]; 8]
}

impl Grid{
    pub fn new() -> Grid{
        return Grid{ currentBuffer: 2, display: 1, one: [[false; 8]; 8], two: [[false; 8]; 8] };
    }

    fn switch_grid(&mut self){
        self.currentBuffer = if self.currentBuffer == 1{ 2 } else{ 1 };
        self.display = if self.display == 1{ 2 } else{ 1 };
    }

    fn valid_cell(x:u32, y:u32)->bool{
        if x < 8 && y < 8{
            return true;
        }
        return false;
    }

    fn num_neighbors(grid:&[[bool;8];8], x:u32, y:u32) -> u8{
        let mut count:u8 = 0;
        /* check each square touching current square */
        for i in (x-1)..(x+1){
            for j in (y-1)..(y+1){
                if Grid::valid_cell(i, j) && grid[i as usize][j as usize] == true{
                    count += 1;
                }
            }
        }
        count
    }

    fn neighbors(&self, x:u32, y:u32) -> u8{
        if self.currentBuffer == 1{
            return Grid::num_neighbors(&self.one, x, y);
        }
        return Grid::num_neighbors(&self.two, x, y);
    }

    pub fn next(&mut self){
        /* apply game of life logic here */
        for i in 0..8{
            for j in 0..8{
                let neighbors = self.neighbors(i,j);
                if neighbors >= 4 || neighbors <= 1{
                    /* kill cell */
                    if self.currentBuffer == 1{
                        self.one[i as usize][j as usize] = false;
                    }
                    else{
                        self.two[i as usize][j as usize] = false;
                    }
                }
                else if neighbors == 3{
                    /* revive cell */
                    if self.currentBuffer == 1{
                        self.one[i as usize][j as usize] = true;
                    }
                    else{
                        self.two[i as usize][j as usize] = true;
                    }
                }
            }
        }
        self.switch_grid();
    }
}
