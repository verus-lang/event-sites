use vstd::prelude::*;
fn main() {}

verus! {

proof fn test(x: int, y: int)
    ensures x + y == y + x
{
}

} // verus!
