use vstd::prelude::*;

verus!{

proof fn test(x: int, y: int)
    ensures x + y == y + x
{
}

}

fn main() { }
