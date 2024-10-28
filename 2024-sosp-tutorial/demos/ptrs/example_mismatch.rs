#![allow(unused_imports)]

use builtin::*;
use builtin_macros::*;
use vstd::simple_pptr::*;
use vstd::prelude::*;

verus!{

fn main() {
    unsafe {
        // ALLOCATE p
        let (p, Tracked(mut perm_p)) = PPtr::<u64>::empty();

        // ALLOCATE q
        let (q, Tracked(mut perm_q)) = PPtr::<u64>::empty();

        // DEALLOCATE p
        p.free(Tracked(perm_p));

        // READ-AFTER-FREE (read from p, try to use q's permission object)
        let x = p.read(Tracked(&mut perm_q)); 
    }
}

}
