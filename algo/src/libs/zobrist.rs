
/* zobrist {{{ */
mod zobrist {
    #![allow(dead_code)]

    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    /* Set {{{ */
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
    pub struct Set<T: Hash> {
        val: u64,
        _marker: std::marker::PhantomData<T>,
    }

    #[allow(dead_code)]
    impl<T: Hash> Set<T> {

        pub fn new() -> Self {
            return Self {val: 0, _marker: std::marker::PhantomData};
        }

        pub fn flip(&mut self, x: &T) {
            let mut hasher = DefaultHasher::new();
            x.hash(&mut hasher);
            let hash = hasher.finish();
            self.val ^= hash;
        }
    }
    /* }}} */


    /* MultiSet {{{ */
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
    pub struct MultiSet<T: Hash> {
        val: u64,
        _marker: std::marker::PhantomData<T>,
    }

    #[allow(dead_code)]
    impl<'a, T: Hash> MultiSet<T> {
        const MOD: u64 = (1 << 61) - 1;

        pub fn new() -> Self {
            return Self {val: 0, _marker: std::marker::PhantomData};
        }

        pub fn insert(&mut self, x: T) {
            let mut hasher = DefaultHasher::new();
            x.hash(&mut hasher);
            let hash = hasher.finish() & Self::MOD;
            self.val = (self.val + hash) & Self::MOD;
        }

        pub fn remove(&mut self, x: &T) {
            let mut hasher = DefaultHasher::new();
            x.hash(&mut hasher);
            let hash = hasher.finish() & Self::MOD;
            self.val = (self.val + Self::MOD - hash) & Self::MOD;
        }

        pub fn extend(&mut self, other: &Self) {
            self.val = (self.val + other.val) & Self::MOD;
        }

        pub fn subtract(&mut self, other: &Self) {
            self.val = (self.val + Self::MOD - other.val) & Self::MOD;
        }
    }
    /* }}} */

}
/* }}} */
