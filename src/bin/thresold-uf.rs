//!Run a Monte Carlo simulation with the union find structure. 
//! Execute it with 
//!```
//!cargo run --bin thresold-uf n nb
//! ```
//!  where `nxn` is the size  of the grid and nb the number of iterations you want to do. 

use isae_project::simulation_uf;
use std::env;

/// Return the proportion of open cells after a Monte Carlo simulation with params passed when running the program.
fn main(){
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 3, "You must provide width size of grid and number of simulations");
    let n = args[1].parse().unwrap(); 
    let nb = args[2].parse().unwrap();
    let thresold = simulation_uf::simulation_uf(n, nb, 1024);
    println!("Thresold Done ! Output value : {}", thresold);
}