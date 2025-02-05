
/* Coord {{{ */
#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
struct Coord<T> {
    x: T, 
    y: T,
}

#[allow(dead_code)]
impl<T: num_traits::real::Real + Clone + Copy> Coord<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    fn dist(&self, other: Self) -> T {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}
/* }}} */
