use vstd::prelude::*;
fn main() {}

verus! {

proof fn axiom_seq_equal(x: Seq<u8>, y: Seq<u8>)
    requires
        ...x and y’s lengths are equal...,
        ...x and y’s elements are equal...,
    ensures
        x == y,
{
    admit()
}

proof fn demand_eq(x: Seq<u8>, y: Seq<u8>)
    requires
        x == y,
{
}

proof fn test_seq_eq() {
    axiom_seq_equal(...);
    demand_eq(
        seq![10] + seq![20, 30],
        seq![10, 20] + seq![30],
    ); // make this succeed
}

} // verus!
