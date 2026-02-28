//!Test `simulation_uf` with a grid with dimensions `300x300` and a seed of $1024$ 
use isae_project::simulation_uf;

#[cfg(test)]
#[test]
fn test_simu_uf(){
    println!("thresold : {}", simulation_uf::simulation_uf(300, 50, 1024));
}