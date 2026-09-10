#![allow(unused_imports)]

use vstd::prelude::*;
use vstd::simple_pptr::*;

verus!{

fn main() {
    unsafe {
        // ALLOCATE p
        let (p, Tracked(mut perm_p)) = PPtr::<u64>::empty();

        // ALLOCATE q
        let (q, Tracked(mut perm_q)) = PPtr::<u64>::empty();

        // DEALLOCATE p
        PPtr::free(p, Tracked(perm_p));

        // READ-AFTER-FREE (read from p, try to use q's permission object)
        // This fails the precondition of PPtr::read
        let x = PPtr::read(p, Tracked(&mut perm_q));
    }
}

}
