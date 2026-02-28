//! Implementation of a percolation Monte Carlo simulation.
use crate::grid::*;
use crate::percolate_dfs::*;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

fn closed_grid(n :usize, m:usize)->Grid{
    let mut array : Vec<Cell> = Vec::new();
    for i in 0..n{
        for j in 0..m{
            let cell = Cell{
                column : j,
                row : i,
                open : false,
                visited : false
            };
            array.push(cell)
        }
    }
    return Grid{
        height : n,
        width : m, 
        grid : array
    }
}


///Run Monte-Carlo simulation where n is the size of the square grid, nb the number of simulations, return the thresold value (ration between the number of open cells and the number of cells)

pub fn simulation_dfs(n: usize, m:usize, nb : u64, seed: u64) -> f64{
    
    let mut rng = StdRng::seed_from_u64(seed);
    let mut p = 0.;
    for iter in 0..nb {
        let mut grid = closed_grid(n, m);
        let mut percolate = false;
        let mut nb_cell_open= 0;
        let nb_cell = n*m;

        while !percolate {
            
            //Put every cell to not visited 
            
            let random_j = rng.random_range(0..m);
            let random_i = rng.random_range(0..n);
            let random_id = id(random_i, random_j, m);
            
            if !grid.grid[random_id].open{ // If cell was closed
                for i in 0..grid.height{
                    for j in 0..grid.width{
                        grid.grid[id(i, j, grid.width)].visited = false;
                    }
                }
                grid.grid[random_id].open = true;
                nb_cell_open += 1; 
                percolate = percolate_dfs(&mut grid);   
            }
            // println!("Avant :");
            // grid.display();
            // println!("Apres :");
            // grid.display();
            
            
        }
        println!("Simulation n°{} : {}", iter+1, (nb_cell_open as f64)/(nb_cell as f64));
        println!("Nb cell open : {}", nb_cell_open);
        p+= (nb_cell_open as f64)/(nb_cell as f64);
        let filename = format!("test-mc-dfs-{}-{}",seed, iter);
        let _= grid.generate_image(&filename);
    }
    return p/(nb as f64);
}
