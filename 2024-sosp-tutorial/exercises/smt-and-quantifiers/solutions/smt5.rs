use vstd::prelude::*;
fn main() {}

verus! {

proof fn test_by_bit_vector(x: u32)
    requires x <= 200,
{
    assert(x >> 8 == 0) by(bit_vector)
        requires(x <= 255);
}

proof fn test_by_nonlinear_arith(x: u32, y: u32)
    requires x > 1 && y > 0
{
    assert(x * y > y) by(nonlinear_arith)
        requires(x > 1 && y > 0);
}

} // verus!
