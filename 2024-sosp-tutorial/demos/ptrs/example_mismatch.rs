#![allow(unused_imports)]

use builtin::*;
use builtin_macros::*;
use vstd::simple_pptr::*;
use vstd::prelude::*;

verus!{

fn main() {
    unsafe {
        // ALLOCATE p
        let (p, Tracked(mut points_to_p)) = PPtr::<u64>::empty();

        // ALLOCATE q
        let (q, Tracked(mut points_to_q)) = PPtr::<u64>::empty();

        // DEALLOCATE p
        p.free(Tracked(points_to_p));

        // READ-AFTER-FREE (read from p, try to use q's permission object)
        let x = p.read(Tracked(&mut points_to_q)); 
    }
}

}
