use std::time::Instant;

use bit_set::BitSet;

use crate::graphs::Graph;

/// This struct is used for computing max clique size of a graph
pub struct Ostergard<'a, T: std::fmt::Display> {
    graph: &'a Graph<T>,
    pub max_cliques: Vec<BitSet>,
    pub max_clique_size: usize,
    max_clique_size_on_suffix: Vec<usize>,
}

impl<'a, T: std::fmt::Display> Ostergard<'a, T> {
    pub fn new(graph: &'a Graph<T>) -> Ostergard<'a, T> {
        Ostergard {
            graph,
            max_clique_size_on_suffix: vec![0; graph.vertices.len()],
            max_cliques: Vec::new(),
            max_clique_size: 0,
        }
    }

    /// Computes max clique size. The result will be stored in the structure
    pub fn compute(&mut self) {
        for i in (0..(self.graph.vertices.len())).rev() {
            let now = Instant::now();

            let mut clique = BitSet::new();
            clique.insert(i);
            let found_clique = self.ostergard(
                &mut clique,
                (i..(self.graph.vertices.len()))
                    .collect::<BitSet>()
                    .intersection(&self.graph.neighbours[i])
                    .collect(),
            );
            if found_clique {
                self.max_clique_size += 1;
            }
            self.max_clique_size_on_suffix[i] = self.max_clique_size;
            println!(
                "progress {}/{}, time: {} s., coeffs: {:?}",
                self.graph.vertices.len() - i,
                self.graph.vertices.len(),
                now.elapsed().as_secs(),
                self.max_clique_size_on_suffix
            );
        }
    }

    /// Internal recursive function which is used to calculate max clique size
    fn ostergard(&self, clique: &mut BitSet, mut candidates: BitSet) -> bool {
        if candidates.is_empty() {
            if clique.len() > self.max_clique_size {
                println!(
                    "Found a clique of size {}\nIndices: {:?}\nVertices:",
                    clique.len(),
                    clique
                );
                for v in clique.iter() {
                    println!("{}", self.graph.vertices[v]);
                }
                return true;
            }
            return false;
        }

        while !candidates.is_empty() {
            if clique.len() + candidates.len() <= self.max_clique_size {
                return false;
            }

            let current = candidates.iter().next().unwrap();

            if clique.len() + self.max_clique_size_on_suffix[current] <= self.max_clique_size {
                return false;
            }

            candidates.remove(current);
            let next_candidates = candidates
                .intersection(&self.graph.neighbours[current])
                .collect();

            clique.insert(current);
            let found_clique = self.ostergard(clique, next_candidates);
            clique.remove(current);

            if found_clique {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    fn build_graph_form_mask(vertices_num: usize, mut mask: u32) -> Graph<usize> {
        let vertices = (0..vertices_num).collect();

        let mut neighbours = vec![BitSet::new(); vertices_num];
        for vertex_1 in 0..vertices_num {
            for vertex_2 in (vertex_1 + 1)..vertices_num {
                if mask & 1 == 1 {
                    neighbours[vertex_1].insert(vertex_2);
                    neighbours[vertex_2].insert(vertex_1);
                }
                mask >>= 1;
            }
        }

        Graph {
            vertices,
            neighbours,
        }
    }

    fn compute_naive_max_clique_size<T>(graph: &Graph<T>) -> usize {
        let n = graph.vertices.len();

        let mut max_clique_size = 0;
        for mask in 0..(1 << n) {
            let clique: Vec<usize> = (0..n)
                .into_iter()
                .filter(|x| (mask >> x) & 1 == 1)
                .collect();
            if clique.len() <= max_clique_size {
                continue;
            }

            let mut found_antiedge = false;
            for (&vertex_1, &vertex_2) in clique.iter().tuple_combinations() {
                if !graph.neighbours[vertex_1].contains(vertex_2) {
                    found_antiedge = true;
                    break;
                }
            }
            if !found_antiedge {
                max_clique_size = clique.len();
            }
        }

        max_clique_size
    }

    #[test]
    fn test_ostergard() {
        for n in 1..=5 {
            for mask in 0..(1 << n * (n - 1) / 2) {
                let graph = build_graph_form_mask(n, mask);

                let naive_clique_size = compute_naive_max_clique_size(&graph);

                let mut ostergard = Ostergard::new(&graph);
                ostergard.compute();

                assert_eq!(ostergard.max_clique_size, naive_clique_size);
            }
        }
    }
}
