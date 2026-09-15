use vstd::prelude::*;

verus!{

//// Minimized sequence library

#[verifier::external_body]
struct Seq { }

uninterp spec fn empty() -> Seq;
uninterp spec fn len(s: Seq) -> nat;
uninterp spec fn push(s: Seq, value: int) -> Seq;
uninterp spec fn index(s: Seq, idx: int) -> int;

axiom fn empty_len()
    ensures len(empty()) == 0;

axiom fn push_len(s: Seq, value: int)
    ensures len(push(s, value)) == len(s) + 1;

//// Example

proof fn test(x: int, y: int) {
    let s = empty();
    let t = push(s, x+y);
    let u = push(t, x-y);

    empty_len();
    push_len(s, x+y);
    push_len(t, x-y);

    assert(len(u) == 2);
}














/// Test with the real Verus sequence library
proof fn test2(x: int, y: int) {
    use vstd::seq::Seq; // get the Verus Seq instead of the Seq defined in this file

    let s = Seq::<int>::empty();
    let t = Seq::push(s, 0);
    let u = Seq::push(t, 0);
    assert(Seq::len(u) == 2);
}



}

fn main() { }
