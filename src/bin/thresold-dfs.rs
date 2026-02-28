//!Run a Monte Carlo simulation with the naive structure. 
//! Execute it with 
//!```
//! cargo run --bin thresold-dfs n m nb
//!``` 
//!where `nxm` is the size of the grid and nb the number of iterations you want to do. 
use isae_project::simulation_dfs;
use std::env;

/// Return the proportion of open cells after a Monte Carlo simulation with params passed when running the program.

fn main(){
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 4, "You must provide width height and number of simulations");
    let width = args[1].parse().unwrap(); 
    let height = args[2].parse().unwrap();
    let nb = args[3].parse().unwrap();
    let thresold = simulation_dfs::simulation_dfs(height, width, nb, 1024);
    println!("Thresold Done ! Output value : {}", thresold);
}