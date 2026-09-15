#![allow(unused_imports)]

use vstd::prelude::*;
use vstd::simple_pptr::*;

verus!{

fn main() {
    let (ptr, Tracked(mut points_to)) = PPtr::<u64>::new(0); // new allocation

    let ptr_1 = ptr; // copy the ptr to create multiple aliases
    let ptr_2 = ptr;

    // write through `ptr_1`
    PPtr::write(ptr_1, Tracked(&mut points_to), 123); // *ptr_1 = 123;

    // write through `ptr_2`
    PPtr::write(ptr_2, Tracked(&mut points_to), 456); // *ptr_2 = 456;

    let x = PPtr::read(ptr_1, Tracked(&points_to)); // let x = *ptr_1;



    // free the allocation
    let y = PPtr::into_inner(ptr, Tracked(points_to)); // let y = *ptr; free(ptr);
}

}
