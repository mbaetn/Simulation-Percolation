//! Test with pointer, too complicated for me, I'll with do it with two tables

/// For which Cell we can access to another Cell (its parent), its position in the structure and its rank. If c.parent == None, then the cell is its own parent 
pub struct Cell{
    pub parent : Option<Box<Cell>>,
    pub rank : u64,
}



pub struct Uf {
    pub uf : Vec<Cell>,
    pub size : usize
}

pub fn id (i: usize, j:usize, w:usize) -> usize{
    return i*w + j; 
}

impl Uf{


    
    pub fn create_uf(nb:usize) -> Self {
        let mut uf = Vec::new();

        for _ in 0..nb {
            let  c = Cell{
                parent : None,
                rank : 0,
            };
            uf.push(c);
        } 
        return Uf {uf : uf, size: nb};
    }


    

    // ///Recursive function with path compression, find the representative of the cell passed in argument

    // pub fn find(&mut self, cell : &mut Cell) -> &mut Cell {
    //     if let _= cell.parent {
    //         cell.parent = Some(self.find(&mut cell.parent));
    //     }
    //     return cell;
    // }

}

