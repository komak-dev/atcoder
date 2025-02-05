
/* ZobristMultiSet {{{ */
#[allow(dead_code)]
#[derive(Clone, Copy)]
struct ZobristMultiSet<'a, T: Eq + std::hash::Hash + Clone> {
    val: u64,
    rand_table: &'a std::cell::RefCell<std::collections::HashMap<T, u64>>,
}

#[allow(dead_code)]
impl<'a, T: Eq + std::hash::Hash + Clone> ZobristMultiSet<'a, T> {
    const MOD: u64 = (1 << 61) - 1;

    fn generate_rand_table() -> std::cell::RefCell<std::collections::HashMap<T, u64>> {
        std::cell::RefCell::new(std::collections::HashMap::<T, u64>::new())
    }

    fn new(rand_table: &'a std::cell::RefCell<std::collections::HashMap<T, u64>>) -> Self {
        return Self {val: 0, rand_table};
    }

    fn insert(&mut self, x: T) {
        use rand::Rng;
        let mut rand_table = self.rand_table.borrow_mut();
        if !rand_table.contains_key(&x) {
            let mut rng = rand::thread_rng();
            rand_table.insert(x.clone(), rng.gen::<u64>() & Self::MOD);
        }
        self.val = (self.val + rand_table[&x]) & Self::MOD;
    }

    fn remove(&mut self, x: &T) {
        self.val = (self.val + Self::MOD - self.rand_table.borrow()[x]) & Self::MOD;
    }

    fn extend(&mut self, other: &Self) {
        self.val = (self.val + other.val) & Self::MOD;
    }

    fn subtract(&mut self, other: &Self) {
        self.val = (self.val + Self::MOD - other.val) & Self::MOD;
    }
}

impl<'a, T: Eq + std::hash::Hash + Clone> PartialEq for ZobristMultiSet<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        return self.val.eq(&other.val);
    }
}

impl<'a, T: Eq + std::hash::Hash + Clone> Eq for ZobristMultiSet<'a, T> {}

impl<'a, T: Eq + std::hash::Hash + Clone> PartialOrd for ZobristMultiSet<'a, T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(self.cmp(other));
    }
}

impl<'a, T: Eq + std::hash::Hash + Clone> Ord for ZobristMultiSet<'a, T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.val.cmp(&other.val);
    }
}

impl<'a, T: Eq + std::hash::Hash + Clone> std::hash::Hash for ZobristMultiSet<'a, T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.val.hash(state);
    }
}
/* }}} */
