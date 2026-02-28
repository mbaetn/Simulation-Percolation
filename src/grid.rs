//! Implementation of the grid structure.
use std::fs::File;
use std::io;
use std::io::BufWriter;
use std::io::prelude::*;

pub struct Cell{
    pub column : usize,
    pub row : usize,
    pub open : bool,
    pub visited : bool
}

pub struct Grid{
    pub height : usize,
    pub width : usize,
    pub grid : Vec<Cell>
}



pub fn id (i: usize, j:usize, w:usize) -> usize{
    return i*w + j; 
}

impl Grid{
    ///Function to display a grid 
    pub fn display(&self){
        for i in 0..self.height{
            for j in 0..self.width{
                let cell_id = id(i, j, self.width);
                let c = &self.grid[cell_id];
                if c.open&&(!c.visited){
                    print!(" ");
                }
                else if c.open{
                    print!("X");
                }
                else{
                    print!("#");
                }
            }
            print!("\n");
        }
    }


    pub fn modif_grid(mut self, i : usize, j : usize, new_cell : Cell){
        self.grid[id(i, j, self.width)] = new_cell;
    }

    pub fn generate_image(&self, filename : &str) -> io::Result<()>{
        let dir = &format!("./out/{}.ppm", filename);
        let file = File::create(dir)?;
        {
            let mut writer = BufWriter::new(file);
            writeln!(writer, "P3\n{} {}\n255", self.width, self.height)?;
            for i in 0..self.height{
                for j in 0..self.width{
                    let cell_id = id(i, j, self.width);
                    let c = &self.grid[cell_id];
                    if c.open&&(!c.visited){
                        write!(writer, "255 255 255 ")?;
                    }
                    else if c.open{
                        write!(writer, "0 0 255 ")?;
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
