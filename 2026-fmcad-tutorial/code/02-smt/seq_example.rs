use vstd::prelude::*;

verus!{

//// Minimized sequence library

#[verifier::external_body]
struct Seq { }

uninterp spec fn empty() -> Seq;
uninterp spec fn len(s: Seq) -> nat;
uninterp spec fn push(s: Seq, value: int) -> Seq;
uninterp spec fn index(s: Seq, idx: int) -> int;

//// Example

proof fn test(x: int, y: int) {
    let s = empty();
    assert(push(s, x+y) == push(s, y+x));
}

}

fn main() { }
