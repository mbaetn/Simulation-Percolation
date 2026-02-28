use isae_project::simulation_dfs;

#[cfg(test)]
#[test]
///Call the `simulation_dfs` function on a `300x300` grid 
/// /!\ It is long to run 
fn test_simu_dfs(){
    let p = simulation_dfs::simulation_dfs(300, 300,2,1024);
    println!("Simulation dfs\n==>thresold : {}", p);
}