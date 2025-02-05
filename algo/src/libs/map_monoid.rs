
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

struct F;
impl ac_library::MapMonoid for F {
    type M = M;
    type F = i32;

    fn identity_map() -> Self::F {
        0
    }

    fn mapping(&f: &Self::F, &x: &<Self::M as ac_library::Monoid>::S) -> <Self::M as ac_library::Monoid>::S {
        f + x
    }

    fn composition(&f: &Self::F, &g: &Self::F) -> Self::F {
        f + g
    }
}
