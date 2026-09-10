#![allow(unused_imports)]

use vstd::prelude::*;
use vstd::simple_pptr::*;

verus!{

fn main() {
    unsafe {
        // ALLOCATE
        // p: PPtr<u64>, points_to: PointsTo<u64>
        let (p, Tracked(mut points_to)) = PPtr::<u64>::empty();

        // unsafe { *p = 5; }
        PPtr::write(p, Tracked(&mut points_to), 5);

        // let x = unsafe { *p };
        let x = PPtr::read(p, Tracked(&points_to)); 

        // DEALLOCATE
        PPtr::free(p, Tracked(points_to));

        // READ-AFTER-FREE
        let x2 = PPtr::read(p, Tracked(&mut points_to)); 
                                    // ^^^^^^^^^^^^^^ borrow check error here
    }
}

}
