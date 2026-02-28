//! This file implements the percolation test with a depth-first search method.
use crate::grid::*;


fn explore(grid : &mut Grid, i : usize, j : usize) -> bool {
    let w = grid.width;
    let cell_id = id(i, j, w);
    // println!("{} {}", i, j);
    grid.grid[cell_id].visited = true;

    //Base case : bottom row 
    if i == grid.height - 1 && grid.grid[cell_id].open{
        return true;
    }
    
    //Explore each open cell neighbour 
    if (i < grid.height - 1)&&(grid.grid[id(i + 1, j, w)].open)&&(!grid.grid[id(i + 1, j, w)].visited) {
        if explore(grid,i + 1, j){
            return true;
        }
    }
    if (j > 0)&&(grid.grid[id(i, j - 1, w)].open)&&(!grid.grid[id(i, j - 1, w)].visited){
        if explore(grid, i, j - 1){
            return true;
        }
    }
    if (j < w - 1)&&(grid.grid[id(i, j + 1, w)].open)&&(!grid.grid[id(i, j + 1, w)].visited){
        if explore(grid, i, j + 1){
            return true;
        }
    }
    if (i > 0)&&(grid.grid[id(i -1, j, w)]).open&&(!grid.grid[id(i -1, j, w)].visited){
        if explore(grid, i - 1, j){
            return true;
        }
    }

    return false;
}


/// Returns true if the grid in argument has a path between the top and the bottom row. The algorithm which is used is the depth-first search.
pub fn percolate_dfs(grid : &mut Grid) -> bool {
    let width = grid.width;

    
    for j in 0..(width){ 
        let  c = &mut grid.grid[j];
        c.visited = true;
        if c.open { 
            // Exlpore open cell of the first line, only looking for bottom neighbour for the first line
            let neigb_id = id(1, j, width);
            
            if grid.grid[neigb_id].open{
                if explore(grid, 1, j){
                    return true;
                }
            }
        }
    }
    return false;
}
