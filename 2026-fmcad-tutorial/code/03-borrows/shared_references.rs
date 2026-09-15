use vstd::prelude::*;

verus!{

fn maximum(a: u64, b: u64) -> (max: u64)
    ensures max >= a && max >= b
{
    if a > b {
        a
    } else {
        b
    }
}

fn maximum_references(a: &u64, b: &u64) -> (max: u64)
    ensures max >= *a && max >= *b
{
    if *a > *b {
        *a
    } else {
        *b
    }
}

}

fn main() { }
