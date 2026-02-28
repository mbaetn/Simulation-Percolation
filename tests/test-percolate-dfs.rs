//! This file test the correctness of the percolation program with the naive 
// method

use isae_project::loader;
use isae_project::percolate_dfs;

#[test]
/// Verify if percolate_dfs works with grids located in ./data/ 
fn test_on_data(){
    //Test  non percolates files. 
    println!("Testing percolate dfs on data...");
    let mut result_np : [bool ; 5] = [false, false, false, false, false];
    let mut result_p : [bool ; 10] = [true, true, true, true, true, true, true, true, true, true];
    for i in 1..=5{
        let filename = format!("./data/grid_np_{}.txt", i);
        let grid = &mut loader::load(&filename);

        // grid.display();
        // println!();
        result_np[i-1] = percolate_dfs::percolate_dfs(grid);

        //grid.display();
        let out = format!("testfile-dfs-np-{}", i);
        let _= grid.generate_image(&out);
        assert_eq!(result_np[i-1], false);
    }
    for i in 1..=10{
        // Test on percolate files. 
        let filename = format!("./data/grid_p_{}.txt", i);
        let grid = &mut loader::load(&filename);

       
        result_p[i-1] = percolate_dfs::percolate_dfs(grid);

        let out = format!("testfile-dfs-p-{}", i);
        let _= grid.generate_image(&out);
        assert_eq!(result_p[i-1], true);
    }

    println!("Percolate dfs worked on all grids !");
    // test-file-dfs.ppm ????
    
}

