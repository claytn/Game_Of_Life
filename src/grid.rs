/* Grid definition that holds both grids; one used as a buffer while other is updated */
pub struct Grid{
    display: [[bool; 8]; 8],
    buffer: [[bool; 8]; 8]
}

impl Grid{
    pub fn new(arr:[[bool; 8]; 8]) -> Grid{
        return Grid{
            display: arr.clone(),
            buffer: arr.clone()
        };
    }

    pub fn display_grid(&self) -> &[[bool; 8]; 8] {
        &self.display
    }

    fn valid_cell(x:i32, y:i32) -> bool {
     (x < 8 && x >= 0 && y < 8 && y >= 0)
    }

    fn num_neighbors(&self, x:u32, y:u32) -> u8 {
        let mut count:u8 = 0;
        let indexX:i32 = x as i32;
        let indexY:i32 = y as i32;
        /* check each square touching current square */
        for i in (indexX-1)...(indexX+1){
            for j in (indexY-1)...(indexY+1){
                if i != indexX || j != indexY{
                    if Grid::valid_cell(i, j) && self.display[i as usize][j as usize]{
                        count += 1;
                    }
                }
            }
        }
        count
    }

    pub fn next(&mut self) {
        /* game of life logic */
        for i in 0..8{
            for j in 0..8{
                let neighbors = self.num_neighbors(i,j);
                if neighbors >= 4 || neighbors <= 1{
                    /* kill cell */
                    self.buffer[i as usize][j as usize] = false;
                }
                else if neighbors == 3{
                    /* revive cell */
                    self.buffer[i as usize][j as usize] = true;
                }
            }
        }

        /* update the display grid with updated buffer contents */
        self.display = self.buffer.clone();
    }
}
