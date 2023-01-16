#[derive(Debug)]
pub struct AdjacencyMatrix<const N: usize> {
    weights: [[i32; N]; N],
}

const INF: i32 = i32::MAX;

impl<const N: usize> AdjacencyMatrix<N> {
    pub fn new() -> Self {
        AdjacencyMatrix {
            weights: [[INF; N]; N],
        }
    }

    /// Add a directed edge.
    pub fn add_edge(&mut self, from: usize, to: usize, weight: i32) {
        self.weights[from][to] = weight;
    }

    /// Add an undirected edge.
    pub fn add_edge_undirected(&mut self, from: usize, to: usize, weight: i32) {
        self.weights[from][to] = weight;
        self.weights[to][from] = weight;
    }

    /// The Floyd Warshall Algorithm is for solving all pairs of shortest-path problems. The problem
    /// is to find the shortest distances between every pair of vertices in a given edge-weighted
    /// directed graph.
    /// Complexity: O(n^3)
    pub fn floyd_warshall(&self) -> [[i32; N]; N] {
        let mut dist = self.weights;
        /* Add all vertices one by one to the set of intermediate vertices.
        ---> Before start of an iteration, we have shortest distances between all pairs of vertices
        such that the shortest distances consider only the vertices in set {0, 1, 2, .. k-1} as
        intermediate vertices.
        ----> After the end of an iteration, vertex no. k is added to the set of intermediate
        vertices and the set becomes {0, 1, 2, .. k}
        */
        for k in 0..N {
            // Pick all vertices as source one by one
            for i in 0..N {
                // Pick all vertices as destination for the above picked source
                for j in 0..N {
                    // If vertex k is on the shortest path from i to j,
                    // then update the value of dist[i][j]
                    let d_kj = unsafe { *dist.get_unchecked(k).get_unchecked(j) };
                    let d_ik = unsafe { *dist.get_unchecked(i).get_unchecked(k) };
                    let d_ij = unsafe { *dist.get_unchecked(i).get_unchecked(j) };
                    if d_kj != INF && d_ik != INF && d_ij > d_ik + d_kj {
                        let entry = unsafe { dist.get_unchecked_mut(i).get_unchecked_mut(j) };
                        *entry = d_ik + d_kj;
                    }
                }
            }
        }
        dist
    }
}

impl<const N: usize> Default for AdjacencyMatrix<N> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_floyd_warshall() {
        let mut g: AdjacencyMatrix<4> = AdjacencyMatrix::new();
        /*
               10
         (0)------->(3)
          |         /|\
        5 |          |
          |          | 1
         \|/   3     |
         (1)------->(2)
        */
        g.add_edge(0, 3, 10);
        g.add_edge(0, 1, 5);
        g.add_edge(1, 2, 3);
        g.add_edge(2, 3, 1);
        for i in 0..4 {
            g.add_edge(i, i, 0);
        }
        let dist = g.floyd_warshall();
        assert_eq!(
            vec![
                [0, 5, 8, 9],
                [INF, 0, 3, 4],
                [INF, INF, 0, 1],
                [INF, INF, INF, 0],
            ],
            dist
        );
    }
}
