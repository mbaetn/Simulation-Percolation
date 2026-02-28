//! Implementation of union-find structure with a rank and table parent. Opens is a table to see which cell is open.
use std::fmt::*;
use std::io;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

pub struct Uf {
    pub ranks : Vec<usize>,
    pub parents : Vec<usize>,
    pub opens : Vec<bool>,
    pub size : usize
}


impl  Display for Uf{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result{
        for e in  0..self.size{
            writeln!(f, "element {}, parent {}, rank {}, open {}", e, self.parents[e], self.ranks[e], self.opens[e])?;
        }
        return Result::Ok(());
    }
}

impl Uf {

    /// If the structure represents a grid, find position in the structure of the grid element in position (i,j).
    pub fn id(&self, i : usize, j : usize, w: usize) -> usize{
        return i*w + j;
    }
    ///Create a new union find structure. 
    ///```
    ///use isae_project::union_find::Uf;
    ///let uf = Uf::create_uf(8);
    ///```
    pub fn create_uf(nb : usize) -> Self{
        let mut p = vec![0;nb];
        let o = vec![false; nb];
        let r = vec![0; nb];
        for i in 0..nb{
            p[i] = i;
        }

        return Uf { ranks : r, parents : p, opens : o, size : nb};
    }


    /// Find representative of element e with  path compression;
    pub fn find(&mut self, e : usize) -> usize{
        if self.parents[e] != e {
            self.parents[e] = self.find(self.parents[e]);
        }
        return self.parents[e];
    }


    /// Merge class of e1 and e2 with union by rank, if the representatives of e1 and e2 have the same rank, it merges on the class of e1
    pub fn merge(&mut self, e1 : usize, e2 : usize){
        let p1 = self.find(e1);
        let r1 = self.ranks[p1];
        let p2 = self.find(e2);
        let r2 = self.ranks[p2];

        assert!(self.parents[p1] == p1 && self.parents[p2] == p2);
        if p1 != p2 {
            if r1 < r2 {
                self.parents[p1] = p2;
            }
            else{
                self.parents[p2] = p1;
                if r2 == r1 {
                    self.ranks[p1] += 1;
                }
            }
        }
    
    }



    pub fn generate_image(&mut self, width : usize, height : usize, filename : &str) -> io::Result<()>{
        let dir = &format!("./out/{}.ppm", filename);
        let file = File::create(dir)?;
        let top_class = self.find(self.size - 2);
        {
            let mut writer = BufWriter::new(file);
            writeln!(writer, "P3\n{} {}\n255", width, height)?;
            for i in 0..height{
                for j in 0..width{
                    let cell_id = self.id(i, j, width);
                    if self.find(cell_id) == top_class{
                        write!(writer, "0 0 255 ")?;
                    }
                    else if self.opens[cell_id]{
                        write!(writer, "255 255 255 ")?;
                    }
                    
                    else{
                        write!(writer, "0 0 0 ")?;
                    }
                }
                write!(writer, "\n")?;
            }

        }


        println!("Image successfully create in {}\n", dir);
        return Ok(());
    }
}