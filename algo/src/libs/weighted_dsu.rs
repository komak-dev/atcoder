
/* WeightedUnionFind {{{ */
struct WeightedDsu {
    parent: Vec<usize>,
    rank: Vec<usize>,
    diff_weight: Vec<i64>, // 親との重みの差
}

impl WeightedDsu {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            diff_weight: vec![0; n], // 初期は全て 0
        }
    }

    fn leader(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let p = self.parent[x];
            self.parent[x] = self.leader(p);
            self.diff_weight[x] += self.diff_weight[p]; // 親の重みを加算
        }
        self.parent[x]
    }

    fn same(&mut self, x: usize, y: usize) -> bool {
        return self.leader(x) == self.leader(y);
    }

    fn merge(&mut self, x: usize, y: usize, w: i64) -> bool {
        let mut root_x = self.leader(x);
        let mut root_y = self.leader(y);
        if root_x == root_y {
            return false;
        }
        let w = w + self.diff_weight[x] - self.diff_weight[y]; // x -> y の重み
        // ランクに基づいてマージ
        if self.rank[root_x] < self.rank[root_y] {
            std::mem::swap(&mut root_x, &mut root_y);
            self.diff_weight[root_y] = -w;
        } else {
            self.diff_weight[root_y] = w;
        }

        self.parent[root_y] = root_x;
        if self.rank[root_x] == self.rank[root_y] {
            self.rank[root_x] += 1;
        }
        true
    }

    // x と y の重みの差を返す
    pub fn diff(&mut self, x: usize, y: usize) -> Option<i64> {
        if !self.same(x, y) {
            return None;
        }
        Some(self.diff_weight[y] - self.diff_weight[x])
    }
}
/* }}} */
