use scanf::sscanf;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use crate::grid::*;


/// Constructs the grid that the file given in argument represent.
pub fn load(filename : &String) -> Grid {

    println!("Charging {}...", filename);
    let file = File::open(filename).expect("No such file or directory");
    let mut reader = BufReader::new(file);
    let mut first_line_buffer = String::new();
    let _ = reader.read_line(&mut first_line_buffer);


    let mut width: usize = 0;
    let mut height: usize = 0;

    let _= sscanf!(&first_line_buffer, "{} {}", width, height);
    let mut array :Vec<Cell> = Vec::new();


    //Constructing the grid structure
    for i in 0..height{
        let mut line_buffer = String::new();
        let _=reader.read_line(&mut line_buffer);
        for j in 0..width{
            let mut open =true;
            let mut visited=false; 
            if let Some(string_state) = line_buffer.chars().nth(j){
                if string_state=='#'{
                    open=false;
                }
                else if string_state==' '{
                    open = true;
                }
                else {
                    visited = true;
                };
                let c : Cell = Cell{
                    column : j,
                    row : i, 
                    open : open,
                    visited : visited
                };
                array.push(c);
            }
        }
    };

    //Result
    let grid : Grid = Grid {
        height : height,
        width : width,
        grid : array
    };
    return grid;
}
