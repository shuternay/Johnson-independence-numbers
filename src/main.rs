use std::env;

mod graphs;
mod ostergard;

fn main() {
    let args: Vec<i32> = env::args().skip(1).map(|x| x.parse().unwrap()).collect();

    let n = args[0] as usize;
    let k = args[1] as usize;
    let t = args[2];
    let fixed_vertices = args[3..].iter().map(|&x| x as usize).collect();
    let graph = graphs::build_johnson_graph_complement(n, k, t, fixed_vertices);

    let mut ostergard = ostergard::Ostergard::new(&graph);
    ostergard.compute();

    println!(
        "Independence number of J_+/-({}, {}, {}) is {}",
        n, k, t, ostergard.max_clique_size
    );
}
