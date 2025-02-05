
/* MonoidDsu {{{ */
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct MonoidDsu<M: ac_library::Monoid> {
    n: usize,
    parent_or_size: Vec<i32>,
    data: Vec<M::S>,
}

#[allow(dead_code)]
impl <M: ac_library::Monoid> MonoidDsu<M> {
    fn new(data: Vec<M::S>) -> Self {
        let n = data.len();
        let parent_or_size = vec![-1; n];
        Self { n, parent_or_size, data }
    }

    fn merge(&mut self, x: usize, y: usize) -> usize {
        assert!(x < self.n && y < self.n);
        let (mut x, mut y) = (self.leader(x), self.leader(y));
        if x == y {
            return x;
        }
        let new_val = M::binary_operation(&self.data[x], &self.data[y]);
        if -self.parent_or_size[x] < -self.parent_or_size[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.parent_or_size[x] += self.parent_or_size[y];
        self.parent_or_size[y] = x as i32;
        self.data[x] = new_val;
        x
    }

    fn same(&mut self, x: usize, y: usize) -> bool {
        assert!(x < self.n && y < self.n);
        self.leader(x) == self.leader(y)
    }

    fn leader(&mut self, x: usize) -> usize {
        assert!(x < self.n);
        if self.parent_or_size[x] < 0 {
            return x;
        }
        self.parent_or_size[x] = self.leader(self.parent_or_size[x] as usize) as i32;
        self.parent_or_size[x] as usize
    }

    fn size(&mut self, x: usize) -> usize {
        assert!(x < self.n);
        let x = self.leader(x);
        -self.parent_or_size[x] as usize
    }

    fn groups(&mut self) -> Vec<Vec<usize>> {
        let mut leader_buf = vec![0; self.n];
        let mut group_size = vec![0; self.n];
        for i in 0..self.n {
            leader_buf[i] = self.leader(i);
            group_size[leader_buf[i]] += 1;
        }
        let mut result = vec![Vec::new(); self.n];
        for i in 0..self.n {
            result[leader_buf[i]].push(i);
        }
        result.into_iter().filter(|x| !x.is_empty()).collect()
    }

    fn get_val(&mut self, x: usize) -> M::S {
        assert!(x < self.n);
        let x = self.leader(x);
        self.data[x].clone()
    }
}
/* }}} */
