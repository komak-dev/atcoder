
/* Edge {{{ */
#[allow(dead_code)]
#[derive(PartialEq, Eq, Copy, Clone)]
struct Edge {
    from: usize,
    to: usize,
    weight: i64,
}

#[allow(dead_code)]
impl Edge {
    fn new(from: usize, to: usize, weight: i64) -> Self {
        return Self {from, to, weight};
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(self.cmp(other));
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.weight.cmp(&other.weight);
    }
}
/* }}} */


/* Graph {{{ */
#[allow(dead_code)]
struct Graph {
    g: Vec<Vec<(usize, i64)>>,
    n: usize,
}

#[allow(dead_code)]
impl Graph {
    fn new(n: usize) -> Self {
        return Self {g: vec![vec![]; n], n};
    }


    /* add_edge {{{ */
    fn add_edge(&mut self, from: usize, to: usize, weight: i64) {
        self.g[from].push((to, weight));
    }
    /* }}} */


    /* bfs {{{ */
    /// Time complexity is O(V + E)
    fn bfs(&self, start: usize) -> Vec<i64> {
        let mut que = std::collections::VecDeque::new();
        let mut dist = vec![-1; self.n];
        que.push_back(start);
        dist[start] = 0;
        while let Some(cur) = que.pop_front() {
            self.g[cur].iter().for_each(|&(to, _weight)| {
                if dist[to] == -1 {
                    dist[to] = dist[cur] + 1;
                    que.push_back(to);
                }
            });
        }
        return dist;
    }
    /* }}} */


    /* dijkstra {{{ */
    /// Returns the shortest path from the start vertex to all other vertices.
    ///
    /// Time complexity is O((V + E) log V)
    fn dijkstra(&self, start: usize) -> Vec<i64> {
        let mut que = std::collections::BinaryHeap::new();
        let mut dist = vec![-1; self.n];
        que.push((0, start));
        dist[start] = 0;
        while let Some((d, cur)) = que.pop() {
            if dist[cur] < d {
                continue;
            }
            self.g[cur].iter().for_each(|&(to, weight)| {
                let nd = d + weight;
                if dist[to] == -1 || dist[to] > nd {
                    dist[to] = nd;
                    que.push((nd, to));
                }
            });
        }
        return dist;
    }
    /* }}} */


    /* floyd_warshall {{{ */
    /// Returns the shortest path between all pairs of vertices.
    ///
    /// Time complexity is O(V^3)
    fn floyd_warshall(&self) -> Vec<Vec<i64>> {
        let mut dist = vec![vec![1 << 60; self.n]; self.n];
        for i in 0..self.n {
            dist[i][i] = 0;
        }
        for from in 0..self.n {
            self.g[from].iter().for_each(|&(to, weight)| {
                dist[from][to] = dist[from][to].min(weight);
            });
        }
        for (k, i, j) in itertools::iproduct!(0..self.n, 0..self.n, 0..self.n) {
            dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
        }
        return dist;
    }
    /* }}} */


    /* scc_dfs {{{ */
    fn scc_dfs(&self, v: usize, used: &mut Vec<bool>, order: &mut Vec<usize>) {
        used[v] = true;
        self.g[v].iter().for_each(|&(nv, _weight)| {
            if !used[nv] {
                self.scc_dfs(nv, used, order);
            }
        });
        order.push(v);
    }
    /* }}} */


    /* scc_rdfs {{{ */
    fn scc_rdfs(&self, v: usize, k: usize, rg: &Vec<Vec<usize>>, used: &mut Vec<bool>, comp: &mut Vec<usize>) {
        used[v] = true;
        comp[v] = k;
        rg[v].iter().for_each(|&nv| {
            if !used[nv] {
                self.scc_rdfs(nv, k, rg, used, comp);
            }
        });
    }
    /* }}} */


    /* scc {{{ */
    /// Returns the strongly connected components of the graph.
    /// The result is a vector of vectors, where each vector contains the vertices of a strongly connected component.
    ///
    /// Time complexity is O(V + E)
    fn scc(&self) -> Vec<Vec<usize>> {
        let mut rg = vec![vec![]; self.n];
        let mut comp = vec![0; self.n];
        let mut order = vec![];
        let mut used = vec![false; self.n];

        for from in 0..self.n {
            self.g[from].iter().for_each(|&(to, _weight)| rg[to].push(from));
        }

        for v in 0..self.n {
            if !used[v] {
                self.scc_dfs(v, &mut used, &mut order);
            }
        }

        used.fill(false);

        let mut k = 0;
        for i in (0..self.n).rev() {
            if !used[order[i]] {
                self.scc_rdfs(order[i], k, &rg, &mut used, &mut comp);
                k += 1;
            }
        }
        let mut result = vec![vec![]; *comp.iter().max().unwrap() + 1];
        for v in 0..self.n {
            result[comp[v]].push(v);
        }
        return result;
    }
    /* }}} */


    /* tsort {{{ */
    /// Returns the topological sort of the graph.
    /// Graph must be a directed acyclic graph.
    ///
    /// Time complexity is O(V + E)
    fn tsort(&self) -> Vec<usize> {
        let mut deg = vec![0; self.n];
        self.g.iter().for_each(|v| {
            v.iter().for_each(|&(to, _weight)| deg[to] += 1);
        });
        let mut que = std::collections::BinaryHeap::new();
        for i in 0..self.n {
            if deg[i] == 0 {
                que.push(i);
            }
        }
        let mut result = vec![];
        while let Some(from) = que.pop() {
            result.push(from);
            self.g[from].iter().for_each(|&(to, _weight)| {
                deg[to] -= 1;
                if deg[to] == 0 {
                    que.push(to);
                }
            });
        }
        return result;
    }
    /* }}} */


    /* kruskal {{{ */
    /// Returns the minimum spanning tree of the graph.
    ///
    /// Time complexity is O(E log V)
    fn kruskal(&self) -> Vec<Edge> {
        use ac_library::Dsu;
        let mut res = vec![];
        let mut es = vec![];
        for from in 0..self.n {
            self.g[from].iter().for_each(|&(to, weight)| {
                es.push(Edge::new(from, to, weight));
            });
        }
        es.sort();
        let mut uf = Dsu::new(self.n);
        es.iter().for_each(|&e| {
            if !uf.same(e.from, e.to) {
                uf.merge(e.from, e.to);
                res.push(e);
            }
        });
        return res;
    }
    /* }}} */


    /* k_times_transition {{{ */
    /// Returns the result of applying the transition function k times.
    ///
    /// Time complexity is O(V log k)
    fn k_times_transition(&self, mut k: u64) -> Vec<usize> {
        let mut dtable = vec![0; self.n];
        let mut ndtable = vec![0; self.n];
        let mut result = vec![0; self.n];
        for v in 0..self.n {
            dtable[v] = self.g[v][0].0;
            result[v] = v;
        }
        while k > 0 {
            if k & 1 == 1 {
                for v in 0..self.n {
                    result[v] = dtable[result[v]];
                }
            }
            for v in 0..self.n {
                ndtable[v] = dtable[dtable[v]];
            }
            std::mem::swap(&mut dtable, &mut ndtable);
            k >>= 1;
        }
        return result;
    }
    /* }}} */

}
/* }}} */
