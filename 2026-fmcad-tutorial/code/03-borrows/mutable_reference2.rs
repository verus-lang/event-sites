use vstd::prelude::*;

verus!{

// example 1

fn add_1(x: &mut u64)
    requires *old(x) < 1000,
    ensures *final(x) == *old(x) + 1,
{
    *x += 1;
}

fn test() {
    let mut a = 0;
    add_1(&mut a);
    assert(a == 1);
}

// example 1 as an "inout" parameter

fn add_1_inout(x_in: u64) -> (x_out: u64)
    requires x_in < 1000,
    ensures x_out == x_in + 1
{
    return x_in + 1;
}

fn test_inout() {
    let mut a = 0;
    let a = add_1_inout(a);
    assert(a == 1);
}

// example 2

fn get_mut_fst<A, B>(pair: &mut (A, B)) -> (ret: &mut A)
    ensures
        *ret == old(pair).0,
        *final(pair) == (*final(ret), old(pair).1),
{
    &mut pair.0
}

fn get_mut_fst_test() {
    let mut p = (10, 20);

    let r = get_mut_fst(&mut p);
    *r = 100;

    assert(p === (100, 20));
}

// example 3

fn vec_index_mut<T>(vec: &mut Vec<T>, i: usize) -> (elem: &mut T)
    requires
        0 <= i < vec.len()
    ensures
        *elem == (*old(vec))[i as int],
        (*final(vec))@ == (*old(vec))@.update(i as int, *final(elem))
{
    &mut vec[i]
}

fn vec_index_mut_test() {
    let mut v = vec![10, 20, 30];
    let elem = vec_index_mut(&mut v, 1);
    *elem = 4000;
    assert(v@ == seq![10, 4000, 30]);
}

}

fn main() { }
