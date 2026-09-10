use vstd::prelude::*;
verus!{

fn smallest_elt(elts: Vec<u64>) -> (result: u64)
    requires
        elts.len() > 0
    ensures
        forall|i:int| 0 <= i < elts.len() ==> elts[i] >= result,
        exists|i:int| 0 <= i < elts.len() && elts[i] == result,
{
    let mut smallest = elts[0];

    let mut j = 1;
    while j < elts.len() 
        // fill in
    {
        if elts[j] < smallest {
            smallest = elts[j];
        }
        j += 1;
    }

    return smallest;
}

fn main() {
    let v = vec![3,1,2];
    let e = smallest_elt(v);
    assert(e == 1);
}

}
