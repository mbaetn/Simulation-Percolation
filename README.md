# TCS1-IN: PercOWLation

---



This is my rust version of the Percolation project. 

## Loader function  

---

To test the loader function execute : 

```bash 
cargo run --bin test-loader ./data/filename
```



## Monte Carlo simulation

---



There are two programs to run a Monte Carlo simulation depending on which algorithm you want to use.

 

To use the naive method execute : 

```bash
cargo run --bin thresold-dfs n m nb
```

where `nxm` is the size of the grid and `nb` the number of iterations you want to run. 



To use the union find structure you can run : 

```bash
cargo run --bin thresold-uf n nb 
```

where `nxn` is the size of the grid and `nb` the number of iterations. 

 

## Tests 

---

You can run tests to verify functions implemented.  Test files are located in the  `./tests` directory. To run all tests execute :

```bash
cargo test --test '*' -- --nocapture
```



>[!Warning]
>
>Testing simulation-dfs is long so don't worry if nothing happens...

