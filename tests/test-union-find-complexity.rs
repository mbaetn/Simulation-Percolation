//! Test the complexity of union find operations merge and find and export results in ./scripts/union_find_complexity.csv


use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use isae_project::union_find::Uf;
use std::time::{Instant, Duration};
use std::fs::File;
use std::io;
use std::io::BufWriter;
use std::io::prelude::*;



#[cfg(test)]

/// For each element in the uf structure call find function and return the time needed.
fn check_find(uf : &mut Uf) -> Duration{
    let start = Instant::now();
    for i in 0..uf.size{
        let _= uf.find(i);
    }
    return start.elapsed();
}


/// Merge cells randomly uf.size times, return the time needed.
fn check_merge(uf : &mut Uf, rng : &mut StdRng) -> Duration{
    let start = Instant::now();
    for _ in 0..uf.size{
        let a = rng.random_range(0..uf.size);
        let b = rng.random_range(0..uf.size);
        // println!("Merged {} and {}", a, b);
        uf.merge(a, b);
    }
    return start.elapsed();
}


#[test]
fn time_uf () -> io::Result<()>{
    let mut n = 10;
    let mut rng = StdRng::seed_from_u64(0);
    
    let file = File::create("scripts/union_find_complexity.csv")?;
    let mut writer = BufWriter::new(file);

    writeln!(writer, "nb, time_to_merge, time_to_find")?;
    println!("nb, time_to_merge, time_to_find");
    while n<=1280{
        let mut uf =  Uf::create_uf(n*n);


        let merge_duration = check_merge(&mut uf, &mut rng);
        // println!("{}", uf);
        let find_duration = check_find(&mut uf);


        writeln!(writer, "{}, {:?}, {:?}", n*n, merge_duration.as_millis(), find_duration.as_millis())?;
        println!("{}, {:?}, {:?}", n*n, merge_duration, find_duration);
        n *= 2;
    };

    return Ok(());
}