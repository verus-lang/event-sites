use vstd::prelude::*;

verus! {


    fn find_elt(elt: u64, elts: Vec<u64>) -> (result: Option<usize>)
    {
        let mut i = 0;
        while i < elts.len() 
        {
            if elts[i] == elt {
                return Some(i);
            }
            i += 1;
        }
        None
    }





}

fn main() {
    println!("Hello, world!");
}


/*

    fn find_elt(elt: u64, elts: Vec<u64>) -> (result: Option<usize>)
        ensures
            match result {
                Some(i) => i < elts.len() && elts[i as int] == elt,
                None => !elts@.contains(elt),
            },
    {
        let mut i = 0;
        while i < elts.len() 
            invariant
                0 <= i <= elts.len(),
                forall |j| 0 <= j < i ==> elts[j] != elt,
        {
            if elts[i] == elt {
                return Some(i);
            }
            i += 1;
        }
        None
    }


*/