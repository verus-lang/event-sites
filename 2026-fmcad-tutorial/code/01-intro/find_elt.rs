#![verifier::exec_allows_no_decreases_clause]
#![verifier::loop_isolation(false)]

use vstd::prelude::*;
verus!{

/// If elt exists, return the index it's at; otherwise return None
fn find_elt(elts: &Vec<u64>, elt: u64) -> (result: Option<usize>)
    ensures
        match result {
            None => ...
            Some(idx) => ...
        }
{
    let mut j = 0;
    while j < elts.len() {
        if elts[j] == elt {
            return Some(j);
        }
        j += 1;
    }

    return None;
}

fn main() {
    let v = vec![3,1,2];
    let e = find_elt(&v, 4);
    assert(e == None);
}

}
