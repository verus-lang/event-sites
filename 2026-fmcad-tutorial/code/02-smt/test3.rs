use vstd::prelude::*;

verus!{

proof fn test3(i: int) -> (r: int)
    ensures r <= 10
{
    if i < 20 {
        return 5;
    } else {
        return 7;
    }
}

}

fn main() { }
