#![allow(unused_imports)]

use builtin::*;
use builtin_macros::*;
use vstd::simple_pptr::*;
use vstd::prelude::*;

verus!{

fn main() {
    unsafe {
        // ALLOCATE
        // p: PPtr<u64>, points_to: PointsTo<u64>
        let (p, Tracked(mut points_to)) = PPtr::<u64>::empty();

        let p2 = p; // pointers are freely copyable

        // Equivalent to:
        // unsafe { *p = 5; }
        p.write(Tracked(&mut points_to), 5); 

        // Equivalent to:
        // let x = unsafe { *p };
        let x = p.read(Tracked(&points_to)); 

        // DEALLOCATE
        p.free(Tracked(points_to));

        // READ-AFTER-FREE
        let x2 = p.read(Tracked(&mut points_to)); 
    }
}

}
