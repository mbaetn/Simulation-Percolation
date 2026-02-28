//! Implementation of a percolation Monte Carlo simulation wiwth an union find structure.
use crate::union_find::*;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;


/// Create a union find structure representing a grid of size n*n + 2. The two last ones are for the class of the top row and bottom row.
fn closed_uf(n : usize) -> Uf {

    let uf = Uf::create_uf(n*n + 2);
    return uf;

}

/// Check if a cell of top row is connected to a cell of bottom row
pub fn percolate(uf : &mut Uf) -> bool{
    let top_class = uf.find(uf.size - 2);
    let bottom_class = uf.find(uf.size - 1);
    return top_class == bottom_class;
}


/// Run a Monte Carlo simulation whith `nb_iter` iteration on a grid of size `n x n`.
pub fn simulation_uf(n : usize, nb_iter : u64, seed : u64) -> f64{
    
    let mut rng = StdRng::seed_from_u64(seed);

    let mut p = 0.;
    let nb_cell = n*n + 2;
    let nb_real_cell = n*n;
    let top_class = nb_cell - 2;
    let bottom_class = nb_cell - 1;
    
    for iter in 0..nb_iter{
        let mut uf = closed_uf(n);
        let mut percol = false;
        let mut nb_cell_open = 0;


        while !percol{
            
            let j = rng.random_range(0..n);
            let i = rng.random_range(0..n);
            // println!("{}{}", i, j);
            let cell_id = uf.id(i, j, n);


            if !uf.opens[cell_id]{ // If cell was initially closed.
                uf.opens[cell_id] = true;
                nb_cell_open += 1;

                // If the cell is in the top/bottom row
                if i == 0{
                    uf.merge(top_class, cell_id);
                }
                else if i == n-1{
                    uf.merge(bottom_class, cell_id);
                }

                //Merge with opens neighbors
                if i>0 && uf.opens[uf.id(i-1, j, n)]{
                    uf.merge(cell_id, uf.id(i-1, j, n));
                }
                if i<n-1 && uf.opens[uf.id(i+1, j, n)]{
                    uf.merge(cell_id, uf.id(i+1, j, n));
                }
                if j>0 && uf.opens[uf.id(i, j-1, n)]{
                    uf.merge(cell_id, uf.id(i, j-1, n));
                }
                if j<n-1 && uf.opens[uf.id(i, j+1, n)]{
                    uf.merge(cell_id, uf.id(i, j+1, n));
                }
                percol = percolate(&mut uf);
            }
        }
        println!("Simulation n°{} : {}", iter+1, (nb_cell_open as f64)/(nb_real_cell as f64));
        println!("Nb cell open : {}", nb_cell_open);
        p+= (nb_cell_open as f64)/(nb_real_cell as f64);

        let filename = format!("test-mc-uf-{}-{}",seed, iter);
        //TODO : generate image for a uf structure;
        let _= uf.generate_image(n, n, &filename);
    }

    return p/(nb_iter as f64);


    
}