use isae_project::union_find::Uf;


#[cfg(test)]
#[test]
fn test_correctness(){
    println!("Testing union find correctness...");
    let n :usize= 10_00;
    let mut uf  = Uf::create_uf(n*n);
    
    for r in 0..n{
        for c in 0..n{
            let cell_id = uf.id(r, c, n);
            let merge_cell = uf.id(0,(r*n + c)%4,n);
            uf.merge(merge_cell, cell_id);
        }
    }

    for r in 0..n{
        for c in 0..n{
            let cell_id = uf.id(r, c, n);
            let merge_cell = uf.id(0, (r*n + c)%4, n);
            assert_eq!(uf.find(merge_cell), uf.find(cell_id));
        }
    }

}