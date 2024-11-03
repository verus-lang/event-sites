use vstd::prelude::*;
fn main() {}

verus! {

spec fn f(x: int) -> bool
    decreases x
{
    x <= 0 || !f(x - 1)
}

proof fn test1() {
    assert(f(0));
    assert(!f(1));
    assert(f(2));
    assert(!f(3));
}

proof fn test2() {
    reveal_with_fuel(f, 4);
    assert(!f(3));
}

} // verus!
