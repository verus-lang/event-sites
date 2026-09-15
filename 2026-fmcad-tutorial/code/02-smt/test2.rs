use vstd::prelude::*;

verus!{

proof fn test2(i: int) -> (r: int)
    ensures r == i + 15
{
    let mut x = i + 3;
    x = x + 5;
    x = x + 7;
    x
}

}

fn main() { }
