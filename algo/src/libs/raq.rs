
/* RAQ (Range Add Query) {{{ */
struct RAQ<T> {
    n: usize,
    data: Vec<T>,
}

#[allow(dead_code)]
impl<T: Clone + std::ops::Add + num::traits::Zero> RAQ<T> {
    fn new(n: usize) -> Self {
        let n = n.next_power_of_two();
        Self {
            n,
            data: vec![T::zero(); n * 2 - 1],
        }
    }

    fn get(&self, i: usize) -> T {
        let mut i = i + self.n - 1;
        let mut value = self.data[i].clone();
        while i > 0 {
            i = (i - 1) / 2;
            value = value + self.data[i].clone();
        }
        value
    }

    fn add(&mut self, i: usize, value: T) {
        self.data[i + self.n - 1] = self.data[i + self.n - 1].clone() + value;
    }

    fn add_range<R: std::ops::RangeBounds<usize>>(&mut self, range: R, value: T) {
        let start = match range.start_bound() {
            std::ops::Bound::Included(&s) => s,
            std::ops::Bound::Excluded(&s) => s + 1,
            std::ops::Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            std::ops::Bound::Included(&e) => e + 1,
            std::ops::Bound::Excluded(&e) => e,
            std::ops::Bound::Unbounded => self.n,
        };
        let mut l = start + self.n - 1;
        let mut r = end + self.n - 1;
        while l < r {
            if l & 1 == 0 {
                self.data[l] = self.data[l].clone() + value.clone();
            }
            if r & 1 == 0 {
                self.data[r - 1] = self.data[r - 1].clone() + value.clone();
            }
            l = l / 2;
            r = (r - 1) / 2;
        }
    }
}
/* }}} */
