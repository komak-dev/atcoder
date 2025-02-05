
/* Ring {{{ */
struct Ring {
    len: usize,
}

#[allow(dead_code)]
impl Ring {
    fn new(len: usize) -> Self {
        Self { len }
    }

    fn dist_right(&self, from: usize, mut to: usize) -> usize {
        if from > to {
            to += self.len;
        }
        to - from
    }

    fn dist_left(&self, from: usize, to: usize) -> usize {
        self.len - self.dist_right(from, to)
    }

    fn contains_right(&self, from: usize, to: usize, subject: usize) -> bool {
        self.dist_right(from, subject) <= self.dist_right(from, to)
    }

    fn contains_left(&self, from: usize, to: usize, subject: usize) -> bool {
        self.dist_left(from, subject) <= self.dist_left(from, to)
    }
}
/* }}} */
