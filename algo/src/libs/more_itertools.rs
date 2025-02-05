
/* more_itertools {{{ */
#[allow(dead_code)]
mod more_itertools {
    use superslice::Ext;

    /* lcs {{{ */
    /// Returns the longest common subsequence of two slices.
    ///
    /// Time complexity is O(n * m) where n and m are the lengths of the input slices.
    ///
    /// # Examples
    /// ```
    /// use more_itertools::lcs;
    ///
    /// let a = vec![1, 2, 3, 4, 5];
    /// let b = vec![3, 4, 5, 6, 7];
    /// let lcs = lcs(&a, &b);
    /// assert_eq!(lcs, vec![3, 4, 5]);
    /// ```
    pub fn lcs<T: Eq + Clone>(a: &[T], b: &[T]) -> Vec<T> {
        let mut dp = vec![vec![0; b.len() + 1]; a.len() + 1];
        for (i, x) in a.iter().enumerate() {
            for (j, y) in b.iter().enumerate() {
                dp[i + 1][j + 1] = if x == y {
                    dp[i][j] + 1
                } else {
                    dp[i + 1][j].max(dp[i][j + 1])
                };
            }
        }

        let mut res = Vec::new();
        let (mut i, mut j) = (a.len(), b.len());
        while i > 0 && j > 0 {
            if dp[i][j] == dp[i - 1][j] {
                i -= 1;
            } else if dp[i][j] == dp[i][j - 1] {
                j -= 1;
            } else {
                res.push(a[i - 1].clone());
                i -= 1;
                j -= 1;
            }
        }

        res.reverse();
        res
    }
    /* }}} */


    /* lis {{{ */
    /// Returns the longest increasing subsequence of a slice.
    /// If `strict` is true, the subsequence is strictly increasing.
    /// Otherwise, it is non-decreasing.
    ///
    /// Time complexity is O(n * log(n)) where n is the length of the input slice.
    ///
    /// # Examples
    /// ```
    /// use more_itertools::lis;
    ///
    /// let a = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    /// let lis = lis(&a, true);
    /// assert_eq!(lis, vec![1, 2, 3, 5]);
    /// let lis = lis(&a, false);
    /// assert_eq!(lis, vec![1, 1, 2, 3, 5]);
    /// ```
    pub fn lis<T: Ord + Clone>(s: &[T], strict: bool) -> Vec<T> {
        let mut dp = Vec::new();
        let mut positions = Vec::new();

        for x in s {
            let pos = if strict {
                dp.lower_bound(x)
            } else {
                dp.upper_bound(x)
            };
            positions.push(pos);
            if pos == dp.len() {
                dp.push(x.clone());
            } else {
                dp[pos] = x.clone();
            }
        }

        let mut res = vec![0; dp.len()];
        
        let mut ri = res.len() as i64 - 1;
        let mut pi = positions.len() as i64 - 1;

        while 0 <= ri && 0 <= pi {
            if positions[pi as usize] == ri as usize {
                res[ri as usize] = pi;
                ri -= 1;
            }
            pi -= 1;
        }

        res.iter().map(|&x| s[x as usize].clone()).collect()
    }
    /* }}} */


    /* set_partition {{{ */
    /// Returns all partitions of a set of size `n` into parts of size `k` or less.
    /// Each part is expressed as a bit set.
    /// `n` must be in the range [1, 16] and `k` must be in the range [1, n].
    ///
    /// Time complexity is O(n * Bell(n, k)) where Bell(n, k) is the number of partitions of a set of size n into k parts.
    ///
    /// # Examples
    /// ```
    /// use more_itertools::set_partition;
    ///
    /// let partitions = set_partition(3, 2);
    /// assert_eq!(partitions, vec![
    ///    vec![b'111],
    ///    vec![b'011, b'100],
    ///    vec![b'101, b'010],
    ///    vec![b'001, b'110],
    /// ]);
    /// ```
    pub fn set_partition(n: u8, k: u8) -> Vec<Vec<u16>> {
        assert!(k > 0 && k <= n && n <= 16);

        fn dfs (
            idx: u8, 
            size: u8,
            cur: &mut Vec<u16>,
            n: u8,
            k: u8,
            res: &mut Vec<Vec<u16>>
        ) {
            for i in 0..=size {
                let nsize;
                if i == size {
                    cur.push(1 << idx);
                    nsize = size + 1;
                } else {
                    cur[i as usize] |= 1 << idx;
                    nsize = size;
                }

                if idx + 1 == n {
                    if nsize <= k {
                        res.push(cur.clone());
                    }
                } else {
                    dfs(idx + 1, nsize, cur, n, k, res);
                }

                if i == size {
                    cur.pop();
                } else {
                    cur[i as usize] &= !(1 << idx);
                }
            }
        }

        let mut res = Vec::new();
        dfs(0, 0, &mut vec![], n, k, &mut res);

        res
    }
    /* }}} */
}
/* }}} */
