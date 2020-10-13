use bit_set::BitSet;
use itertools::Itertools;
use ndarray::Array1;

pub struct Graph<T> {
    pub vertices: Vec<T>,
    pub neighbours: Vec<BitSet>,
}

/// Generates complement of the generalised Johnson graph J_{+/-}(n, k, t).
///
/// This method allows to provide a list of fixed vertices, which are going to be included in the
/// independent set. Edges from neighbours of these vertices will be excluded from the graph.
pub fn build_johnson_graph_complement(
    n: usize,
    k: usize,
    t: i32,
    fixed_vertices: Vec<usize>,
) -> Graph<Array1<i8>> {
    // Generate list of binom(n, k) supports
    // println!("Supports:");
    let mut supports: Vec<Vec<bool>> = Vec::new();
    for support_positions in (0..n).combinations(k) {
        let mut support: Vec<bool> = vec![false; n];
        for position in support_positions {
            support[position] = true;
        }
        // println!(
        //     "{:?}",
        //     support.iter().map(|&x| x as i8).collect::<Vec<i8>>()
        // );
        supports.push(support);
    }

    // Generate list of 2^k binom(n, k) vertices
    println!("Vertices:");
    let mut vertices: Vec<Array1<i8>> = Vec::new();
    for support in supports {
        for mut mask in 0..(1 << k) {
            let mut vector = vec![0; n as usize];
            for (position, &value) in support.iter().enumerate() {
                if value {
                    vector[position] = match mask & 1 {
                        0 => 1,
                        1 => -1,
                        _ => unreachable!(),
                    };
                    mask >>= 1;
                }
            }
            vertices.push(Array1::from(vector));
        }
    }

    let mut neighbours = Vec::new();
    for (index_1, vertex_1) in vertices.iter().enumerate() {
        neighbours.push(BitSet::new());

        // Skip neighbours of fixed vertices
        let mut dropped = false;
        for &fixed_vertex in fixed_vertices.iter() {
            if vertex_1.dot(&vertices[fixed_vertex]) == t as i8 {
                dropped = true;
                break;
            }
        }
        if dropped {
            continue;
        }

        for (index_2, vertex_2) in vertices.iter().enumerate() {
            // Skip neighbours of fixed vertices
            let mut droped = false;
            for &fixed_vertex in fixed_vertices.iter() {
                if vertex_2.dot(&vertices[fixed_vertex]) == t as i8 {
                    droped = true;
                    break;
                }
            }
            if droped {
                continue;
            }

            // there is an anti-edge between vertex_1 and vertex_2
            if index_1 != index_2 && vertex_1.dot(vertex_2) != t as i8 {
                neighbours[index_1].insert(index_2);
            }
        }

        println!("{} {}", index_1, vertex_1);
    }

    Graph {
        vertices,
        neighbours,
    }
}
