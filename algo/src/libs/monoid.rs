
struct M;
impl ac_library::Monoid for M {
    type S = i32;
    fn identity() -> Self::S {
        0
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        a + b
    }
}
