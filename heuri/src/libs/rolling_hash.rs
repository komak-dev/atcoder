
/* RollingHash {{{ */
use ac_library::{ModInt998244353 as Mint1, ModInt1000000007 as Mint2};

#[derive(Copy, Clone, Debug, Hash)]
struct RHash (Mint1, Mint2);

impl PartialEq for RHash {
    fn eq(&self, other: &Self) -> bool {
        return self.0.val() == other.0.val() && self.1.val() == other.1.val();
    }
}

impl Eq for RHash {}

impl PartialOrd for RHash {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(self.cmp(other));
    }
}

impl Ord for RHash {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return (self.0.val(), self.1.val()).cmp(&(other.0.val(), other.1.val()));
    }
}


#[derive(Clone, Debug)]
struct RollingHash {
    hash: Vec<RHash>,
    power: Vec<RHash>,
}

#[allow(dead_code)]
impl RollingHash {
    const BASE: i64 = 31;

    fn new(s: &String) -> Self {
        let s = s.chars().collect_vec();
        let n = s.len();
        let mut hash = vec![RHash(Mint1::from(0), Mint2::from(0)); n + 1];
        let mut power = vec![RHash(Mint1::from(1), Mint2::from(1)); n + 1];

        for i in 0..n {
            hash[i + 1].0 = hash[i].0 * Self::BASE + s[i] as u8;
            hash[i + 1].1 = hash[i].1 * Self::BASE + s[i] as u8;
            power[i + 1].0 = power[i].0 * Self::BASE;
            power[i + 1].1 = power[i].1 * Self::BASE;
        }

        return Self {hash, power};
    }

    fn get_hash(&self, l: usize, r: usize) -> RHash {
        return RHash(
            self.hash[r].0 - self.hash[l].0 * self.power[r - l].0,
            self.hash[r].1 - self.hash[l].1 * self.power[r - l].1
        );
    }
}
/* }}} */
