use vstd::prelude::*;
fn main() {}

verus! {

#[verifier::external_body]
struct MySeq;

impl MySeq {
    spec fn empty() -> MySeq;
    spec fn len(self) -> nat;
    spec fn push(self, value: int) -> MySeq;
}

proof fn test(x: int, y: int) {
    let s = MySeq::empty();
    assert(
        s.push(x + y)
     == s.push(y + x)
    );
}



/*
proof fn axiom_my_seq_empty()
    ensures
        MySeq::empty().len() == 0,
{
    admit();
}

proof fn axiom_my_seq_push_len(s: MySeq, value: int)
    ensures
        s.push(value).len() == s.len() + 1,
{
    admit();
}

proof fn test(x: int, y: int) {
    let s0 = MySeq::empty();
    let s1 = s0.push(x + y);
    let s2 = s1.push(x - y);
    axiom_my_seq_empty();
    axiom_my_seq_push_len(s0, x + y);
    axiom_my_seq_push_len(s1, x - y);
    assert(s2.len() == 2);
}
*/



/*
broadcast proof fn axiom_my_seq_push_len(s: MySeq, value: int)
    ensures
        #[trigger] s.push(value).len() == s.len() + 1,
{
    admit();
}

proof fn axiom_my_seq_push_len_quant()
    ensures
        forall|s: MySeq, value: int| #[trigger] s.push(value).len() == s.len() + 1,
{
    admit();
}

proof fn test(x: int, y: int) {
    let s0 = MySeq::empty();
    let s1 = s0.push(x + y);
    let s2 = s1.push(x - y);
    axiom_my_seq_empty();
    axiom_my_seq_push_len_quant();
    assert(s2.len() == 2);
}
*/



/*
broadcast proof fn axiom_my_seq_empty()
    ensures
        #[trigger] MySeq::empty().len() == 0,
{
    admit();
}

broadcast proof fn axiom_my_seq_push_len(s: MySeq, value: int)
    ensures
        #[trigger] s.push(value).len() == s.len() + 1,
{
    admit();
}

proof fn test(x: int, y: int) {
    broadcast use axiom_my_seq_empty;
    broadcast use axiom_my_seq_push_len;

    let s0 = MySeq::empty();
    let s1 = s0.push(x + y);
    let s2 = s1.push(x - y);
    assert(s2.len() == 2);
}
*/


} // verus!
