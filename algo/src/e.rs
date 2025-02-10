/* imports {{{ */
#![allow(unused_imports)]
use itertools::{chain, iproduct, izip, Itertools};
use proconio::{
    input, input_interactive,
    marker::{Bytes, Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use superslice::Ext;
use ac_library::*;
/* }}} */


fn main() {
    input! {
        n: usize, m: usize,
        cables: [(Usize1, Usize1); m],
    }

    let mut uf = Dsu::new(n);
    let mut unused = vec![];

    for (i, &(a, b)) in cables.iter().enumerate() {
        if uf.same(a, b) {
            unused.push(i);
        } else {
            uf.merge(a, b);
        }
    }

    let mut leaders = uf.groups().iter().map(|x| uf.leader(x[0])).collect::<BTreeSet<_>>();
    println!("{}", leaders.len() - 1);

    leaders.remove(&uf.leader(0));
    let mut collected = vec![uf.leader(0)];

    for i in 0..leaders.len() {
        let cable = unused[i];
        if leaders.contains(&uf.leader(cables[cable].0)) {
            println!("{} {} {}", cable + 1, cables[cable].0 + 1, collected[0] + 1);
            collected.push(cable);
            leaders.remove(&uf.leader(cables[cable].0));
        } else {
            let to = leaders.pop_first().unwrap();
            uf.merge(cables[cable].0, to);
            println!("{} {} {}", cable + 1, cables[cable].0 + 1, to + 1);
        }
    }
}
