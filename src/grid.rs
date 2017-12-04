/* Grid definition that holds both grids; one used as a buffer while other is updated */
pub struct Grid{
    display: Vec<BitVec>,
    buffer: Vec<BitVec>
}

impl Grid{
    pub fn new(arr:Vec<BitVec>) -> Grid{
        Grid {
            display: arr.clone(),
            buffer: arr.clone()
        }
    }

    pub fn display_grid(&self) -> &[[bool; 8]; 8] {
        &self.display
    }

    fn valid_cell(x:i32, y:i32) -> bool {
     (x < 8 && x >= 0 && y < 8 && y >= 0)
    }

    fn num_neighbors(&self, x:u32, y:u32) -> u8 {
        let mut count:u8 = 0;
        let index_x:i32 = x as i32;
        let index_y:i32 = y as i32;
        /* check each square touching current square */
        for i in (index_x - 1)...(index_x + 1){
            for j in (index_y - 1)...(index_y + 1){
                if i != index_x || j != index_y{
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

        // TODO: FINISH REVISED VERSION HERE.
        self.buffer.iter().for_each(|bit_vec, i| {
            bit_vec.iter().map(|map_entry, j| {
                let neighbors = self.num_neighbors(i, j);
                if neighbors >= 4 || neighbors <= 1 {
                    /* kill cell */
                    return false;
                    //self.buffer[i as usize][j as usize] = false;
                }
                else if neighbors == 3 {
                    /* revive cell */
                    return true;
                    //self.buffer[i as usize][j as usize] = true;
                }
            });
        });
        /*
        for i in 0..8{
            for j in 0..8{

            }
        }*/

        /* update the display grid with updated buffer contents */
        self.display = self.buffer.clone();
    }
}
