
/* linear_programming {{{ */
/// ax + by >= c を満たす (x, y) で、px + qy が最小となるものを計算する
/// O(log(a + b))
fn linear_programming(a: i64, b: i64, c: i64, p: i64, q: i64) -> (i64, i64) {
    let mut min_cost = i64::MAX;
    let (mut x, mut y) = (0, 0);

    for tmp_x in 0..b {
        let tmp_y = num::integer::div_ceil(0.max(c - a * tmp_x), b);
        let cost = p * tmp_x + q * tmp_y;
        if cost < min_cost {
            min_cost = cost;
            (x, y) = (tmp_x, tmp_y);
        }
    }
    for tmp_y in 0..a {
        let tmp_x = num::integer::div_ceil(0.max(c - b * tmp_y), a);
        let cost = p * tmp_x + q * tmp_y;
        if cost < min_cost {
            min_cost = cost;
            (x, y) = (tmp_x, tmp_y);
        }
    }

    (x, y)
}
/* }}} */
