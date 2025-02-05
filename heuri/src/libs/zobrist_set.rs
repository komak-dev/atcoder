
/* ZobristSet {{{ */
#[allow(dead_code)]
#[derive(Clone, Copy)]
struct ZobristSet<'a, T: Eq + std::hash::Hash + Clone> {
    val: u64,
    rand_table: &'a std::cell::RefCell<std::collections::HashMap<T, u64>>,
}

#[allow(dead_code)]
impl<'a, T: Eq + std::hash::Hash + Clone> ZobristSet<'a, T> {

    fn generate_rand_table() -> std::cell::RefCell<std::collections::HashMap<T, u64>> {
        std::cell::RefCell::new(std::collections::HashMap::<T, u64>::new())
    }

    fn new(rand_table: &'a std::cell::RefCell<std::collections::HashMap<T, u64>>) -> Self {
        return Self {val: 0, rand_table };
    }

    fn flip(&mut self, x: &T) {
        use rand::Rng;
        let mut rand_table = self.rand_table.lock().unwrap();
        if !rand_table.contains_key(x) {
            let mut rng = rand::thread_rng();
            rand_table.insert(x.clone(), rng.gen::<u64>());
        }
        self.val ^= rand_table[x];
    }
}

impl<'a, T: Eq + std::hash::Hash + Clone> PartialEq for ZobristSet<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        return self.val.eq(&other.val);
    }
}

impl<'a, T: Eq + std::hash::Hash + Clone> Eq for ZobristSet<'a, T> {}

impl<'a, T: Eq + std::hash::Hash + Clone> PartialOrd for ZobristSet<'a, T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(self.cmp(other));
    }
}

impl<'a, T: Eq + std::hash::Hash + Clone> Ord for ZobristSet<'a, T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.val.cmp(&other.val);
    }
}

impl<'a, T: Eq + std::hash::Hash + Clone> std::hash::Hash for ZobristSet<'a, T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.val.hash(state);
    }
}
/* }}} */
