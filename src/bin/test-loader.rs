//! Test if loader works with the file passed in argument. Use 
//!```
//!cargo run --bin test-loader ./data/filename
//!```
//!to run the program

use std::env;
use isae_project::loader;


/// Load the file and generate an image.
fn main (){
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "You must provide a file name");
    let filename = &args[1];

    let grid = loader::load(filename);
    grid.display();
    let _ = grid.generate_image("image1");
    
}