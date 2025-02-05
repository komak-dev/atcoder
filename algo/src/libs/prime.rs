
/* prime {{{ */
#[allow(dead_code)]
mod prime {


    /* modmul {{{ */
    pub fn modmul(a: i64, b: i64, m: i64) -> i64 {
        return ((a as i128 * b as i128) % m as i128) as i64;
    }
    /* }}} */


    /* modpow {{{ */
    pub fn modpow(mut a: i64, mut n: i64, m: i64) -> i64 {
        let mut res = 1;
        while n > 0 {
            if n & 1 == 1 {
                res = modmul(a, res, m);
            }
            a = modmul(a, a, m);
            n >>= 1;
        }
        return res;
    }
    /* }}} */


    /* modinv {{{ */
    pub fn modinv(mut a: i64, m: i64) -> i64 {
        let mut b = m;
        let mut u = 1;
        let mut v = 0;
        while b > 0 {
            let t = a / b;
            a -= t * b;
            std::mem::swap(&mut a, &mut b);
            u -= t * v;
            std::mem::swap(&mut u, &mut v);
        }
        u %= m;
        if u < 0 {
            u += m;
        }
        return u;
    }
    /* }}} */


    /* eratosthenes {{{ */
    /// Sieve of Eratosthenes
    ///
    /// Time complexity is O(n log log n)
    pub fn eratosthenes(n: u64) -> Vec<u64> {
        let mut is_prime = vec![true; n as usize + 1];
        let mut primes = vec![];
        for i in 2..=n {
            if is_prime[i as usize] {
                for j in ((i * i)..=n).step_by(i as usize) {
                    is_prime[j as usize] = false;
                }
                primes.push(i);
            }
        }
        return primes;
    }
    /* }}} */


    /* is_prime {{{ */
    /// Trial division
    ///
    /// Time complexity is O(n^(1/2))
    pub fn is_prime(n: i64) -> bool {
        let mut i = 2;
        while i * i <= n {
            if n % i == 0 {
                return false;
            }
            i += 1;
        }
        return true;
    }
    /* }}} */


    /* fast_is_prime {{{ */
    /// Miller-Rabin primality test
    ///
    /// Time complexity is O(log n)
    pub fn fast_is_prime(n: i64) -> bool {
        if n == 2 { return true; }
        if n < 2 || n & 1 == 0 { return false; }
        let n1 = n - 1;
        let s = n1.trailing_zeros();
        let d = n1 >> s;
        [2, 325, 9375, 28178, 450775, 9780504, 1795265022].iter().all(|&base| {
            let a = if base < n { 
                base 
            } else { 
                base % n 
            };
            if a == 0 { 
                return true; 
            }
            let mut t = modpow(a, d, n);
            if t == 1 || t == n1 {
                return true; 
            }
            for _ in 1..s {
                t = modmul(t, t, n); 
                if t == n1 { 
                    return true; 
                } 
            }
            false
        })
    }
    /* }}} */


    /* find_factor {{{ */
    /// Pollard's rho algorithm
    pub fn find_factor(n: i64) -> i64 {
        if n & 1 == 0 {
            return 2;
        }
        let mut x = 0;
        let mut y = 0;
        let mut prod = 1;
        let f = |o| modmul(o, o, n) + 1;
        let mut t = 30;
        let mut z = 0;
        while t % 64 != 0 || num::integer::gcd(prod, n) == 1 {
            if x == y {
                z += 1;
                x = z;
                y = f(x);
            }
            let q = modmul(prod, x + n - y, n);
            if q != 0 {
                prod = q;
            }
            x = f(x);
            y = f(f(y));
            t += 1;
        }
        return num::integer::gcd(prod, n);
    }
    /* }}} */


    /* factorize {{{ */
    /// Trial division
    ///
    /// Time complexity is O(n^(1/2))
    pub fn factorize(mut n: i64) -> Vec<i64> {
        let mut factors = vec![];
        let mut p = 2;
        while p * p <= n {
            while n % p == 0 {
                n /= p;
                factors.push(p);
            }
            p += 1;
        }
        if n > 1 {
            factors.push(n);
        }
        return factors;
    }
    /* }}} */


    /* fast_factorize {{{ */
    /// Pollard's rho algorithm
    ///
    /// Time complexity is O(n^(1/4))
    pub fn fast_factorize(n: i64) -> Vec<i64> {
        let mut factors = vec![];
        let mut st = vec![];
        st.push(n);
        while let Some(top) = st.pop() {
            if top == 1 {
                continue;
            }
            if fast_is_prime(top) {
                factors.push(top);
                continue;
            }
            let factor = find_factor(top);
            st.push(factor);
            st.push(top / factor);
        }
        factors.sort();
        return factors;
    }
    /* }}} */


}
/* }}} */
